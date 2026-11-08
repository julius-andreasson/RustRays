use std::time::Instant;

use crate::camera::camera::Camera;
use crate::types::util::{rnd, rnd_range};
use crate::types::vec3::Vec3;
use crate::hittables::hittables::{HittableList, Material, Sphere};
mod types;
mod hittables;
mod camera;

fn main() {
    println!("Building scene...");
    let time_before = Instant::now();

    // Scene
    let mut scene = HittableList::new();

    // Materials
    let mat_ground = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_ground)));

    let p1 = Vec3::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rnd();
            let center = Vec3::new(a as f64 + 0.9*rnd(), 0.2, b as f64 + 0.9*rnd());
            if (center - p1).length_squared() > 0.81 {
    //         if ((center - point3(4, 0.2, 0)).length() > 0.9) {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::rnd() * Vec3::rnd();
                    let sphere_material = Material::Lambertian { albedo };
                    scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::rnd_range(0.5, 1.0);
                    let sphere_material = Material::Metal { albedo, fuzz: rnd_range(0.0, 0.5) };
                    scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Material::Dielectric { refraction_index: 1.5 };
                    scene.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let mat1 = Material::Dielectric { refraction_index: 1.5 };
    scene.add(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
    scene.add(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    scene.add(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    // let cam = Camera::new(1200, 675, 10, 10);
    // let cam = Camera::new(1200, 675, 500, 50);
    // let cam = Camera::new(400, 300, 100, 50);
    let cam = Camera::new(100, 100, 10, 10);
    println!("Starting render...");
    let res = cam.render(&scene);
    if res.is_err() {
        println!("Error: {:#?}", res);
    } else {
        println!("Execution finished.");
    }
    let elapsed_time = time_before.elapsed();
    println!("Total time: {:?}", elapsed_time);
}
