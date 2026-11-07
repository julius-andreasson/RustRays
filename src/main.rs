use crate::camera::camera::Camera;
use crate::types::util;
use crate::types::vec3::Vec3;
use crate::hittables::hittables::{HittableList, Material, Sphere};
mod types;
mod hittables;
mod camera;

fn main() {
    println!("Execution started.");

    // Materials
    let mat_ground = Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) };
    let mat_center = Material::Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) };
    let mat_left = Material::Dielectric { refraction_index: 1.5 };
    let mat_bubble = Material::Dielectric { refraction_index: 1.0/1.5 };
    let mat_right = Material::Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 1.0 };

    let s1 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    let s2 = Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, mat_center);
    let s3 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    let s4 = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble);
    let s5 = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    // Scene
    let mut scene = HittableList::new();
    scene.add(&s1);
    scene.add(&s2);
    scene.add(&s3);
    scene.add(&s4);
    scene.add(&s5);

    let cam = Camera::new(300, 200, 10, 10);
    // let cam = Camera::new(400, 300, 100, 50);
    let res = cam.render(&scene);
    if res.is_err() {
        println!("Error: {:#?}", res);
    } else {
        println!("Execution finished.");
    }
}
