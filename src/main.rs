use crate::camera::camera::Camera;
use crate::types::vec3::Vec3;
use crate::hittables::hittables::{HittableList, Sphere};
mod types;
mod hittables;
mod camera;

fn main() {
    println!("Execution started.");

    // Scene
    let mut scene = HittableList::new();
    let s1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let s2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    scene.add(&s1);
    scene.add(&s2);

    let cam = Camera::new(100, 100);
    cam.render(&scene);
}
