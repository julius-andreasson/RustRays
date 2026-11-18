use crate::img::{gen_and_save};

mod util;
mod types;
mod hittables;
mod camera;
mod texture;

fn main() {
    // gui::run_frame();
    gen_and_save();
}


mod img {
    use std::sync::Arc;
    use std::time::Instant;

    use crate::camera::camera::Camera;
    use crate::texture::texture::{CheckerTexture, ImageTexture};
    use crate::util::util::{rnd, rnd_range};
    use crate::types::vec3::{Color, Vec3};
    use crate::hittables::hittables::{BVHNode, HittableList, Material, Sphere};
    
    pub fn gen_and_save() {
        let img = gen_img();
        let _ = img.save("output/test.jpg");
    }

    pub fn gen_img() -> image::RgbImage {
        println!("Building scene...");
        let time_before = Instant::now();

        // Scene
        // Produce scene by splitting objects into BVHNodes
        let (scene, cam): (BVHNode, Camera) = gen_scene(Scenes::Checker);

        println!("Starting render...");
        let res = cam.render(&scene);
        let elapsed_time = time_before.elapsed();
        println!("Total time: {:?}", elapsed_time);
        return res;
    }
    
    enum Scenes {
        Base,
        Checker,
        Globe,
    }

    fn gen_scene(choice: Scenes) -> (BVHNode, Camera) {
        let mut world: HittableList = HittableList::new();
        let cam: Camera;

        match choice {
            Scenes::Base => {
                // Materials
                let checker = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                let mat_checker = Material::Lambertian { tex: checker };
                world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_checker));

                let p = Vec3::new(4.0, 0.2, 0.0);
                for a in -11..11 {
                    for b in -11..11 {
                        let choose_mat = rnd();
                        let center = Vec3::new(a as f64 + 0.9*rnd(), 0.2, b as f64 + 0.9*rnd());
                        if (&center - &p).length_squared() > 0.81 {
                            if choose_mat < 0.8 {
                                // diffuse
                                let albedo = Vec3::rnd() * Vec3::rnd();
                                let sphere_material = Material::Lambertian { tex: Material::from_color(albedo) };
                                world.add(Sphere::new(center, 0.2, sphere_material));
                            } else if choose_mat < 0.95 {
                                // metal
                                let albedo = Vec3::rnd_range(0.5, 1.0);
                                let sphere_material = Material::Metal { albedo, fuzz: rnd_range(0.0, 0.5) };
                                world.add(Sphere::new(center, 0.2, sphere_material));
                            } else {
                                // glass
                                let sphere_material = Material::Dielectric { refraction_index: 1.5 };
                                world.add(Sphere::new(center, 0.2, sphere_material));
                            }
                        }
                    }
                }

                let mat1 = Material::Dielectric { refraction_index: 1.5 };
                world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1));

                let mat2 = Material::Lambertian { tex: Material::from_color(Color::new(0.4, 0.2, 0.1)) };
                world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2));

                let mat3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
                world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3));

                // Camera
                let width = 400;
                let height = 300;
                let samples_per_pixel = 2;
                let max_depth = 3;
                let vfov = 20.0;
                let lookfrom = Vec3::new(13.0, 2.0, 3.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.6;
                let focus_dist = 10.0;
                
                cam = Camera::new(width, height, samples_per_pixel, max_depth, vfov, lookfrom, lookat, defocus_angle, focus_dist);
            },
            Scenes::Checker => {
                // Materials
                // let checker = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                // let mat_checker = Material::Lambertian { tex: checker };
                let duck = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
                let mat_duck = Material::Lambertian { tex: duck };
                world.add(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, mat_duck));
                let checker_img_tex = Arc::new(ImageTexture::new("textures/checker.jpg"));
                let mat_checker_2 = Material::Lambertian { tex: checker_img_tex };
                world.add(Sphere::new(Vec3::new(0.0, 5.0, 0.0), 5.0, mat_checker_2));

                // Camera
                let width = 400;
                let height = 300;
                let samples_per_pixel = 2;
                let max_depth = 3;
                let vfov = 20.0;
                let lookfrom = Vec3::new(40.0, -10.0, 40.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                
                cam = Camera::new(width, height, samples_per_pixel, max_depth, vfov, lookfrom, lookat, defocus_angle, focus_dist);
            },
            Scenes::Globe => {
                // Materials
                let earth_tex = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
                let mat = Material::Lambertian { tex: earth_tex };
                world.add(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, mat));

                // Camera
                let width = 400;
                let height = 300;
                let samples_per_pixel = 2;
                let max_depth = 3;
                let vfov = 20.0;
                let lookfrom = Vec3::new(0.0, 0.0, 12.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                
                cam = Camera::new(width, height, samples_per_pixel, max_depth, vfov, lookfrom, lookat, defocus_angle, focus_dist);
            },
        }

        (BVHNode::new(world.objects), cam)
    }
}
