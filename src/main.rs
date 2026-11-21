use crate::img::{gen_and_save};

mod util;
mod types;
mod hittables;
mod camera;
mod texture;

fn main() {
    gen_and_save();
}


mod img {
    use std::sync::Arc;
    use std::time::Instant;

    use crate::camera::camera::Camera;
    use crate::texture::texture::{CheckerTexture, ImageTexture};
    use crate::util::util::{rnd, rnd_range};
    use crate::types::vec3::{Color, Vec3};
    use crate::hittables::hittables::{BVHNode, HittableList, Material, Shape2D, Shape2DVariant, Sphere};
    
    pub fn gen_and_save() {
        let img = gen_img();
        let _ = img.save("output/test.jpg");
    }

    static WIDTH: u32 = 400;
    static HEIGHT: u32 = 400;
    static SAMPLES_PER_PIXEL: u32 = 5;
    static MAX_DEPTH: i32 = 5;

    pub fn gen_img() -> image::RgbImage {
        println!("Building scene...");
        let time_before = Instant::now();

        // Scene
        // Produce scene by splitting objects into BVHNodes
        let (scene, cam): (BVHNode, Camera) = gen_scene(Scenes::CornellBox);

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
        Quads,
        SimpleLight,
        CornellBox,
        Test,
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
                                let sphere_material = Material::Lambertian { tex: Material::tex_from_color(albedo) };
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

                let mat2 = Material::Lambertian { tex: Material::tex_from_color(Color::new(0.4, 0.2, 0.1)) };
                world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2));

                let mat3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };
                world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3));

                // Camera
                let vfov = 20.0;
                let lookfrom = Vec3::new(13.0, 2.0, 3.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.6;
                let focus_dist = 10.0;
                let background = Color::new(0.7, 0.8, 1.0);
                
                cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
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
                let vfov = 20.0;
                let lookfrom = Vec3::new(40.0, -10.0, 40.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                let background = Color::new(0.7, 0.8, 1.0);
                
                cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
            },
            Scenes::Globe => {
                // Materials
                let earth_tex = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
                let mat = Material::Lambertian { tex: earth_tex };
                world.add(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, mat));

                // Camera
                let vfov = 20.0;
                let lookfrom = Vec3::new(0.0, 0.0, 12.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                let background = Color::new(0.7, 0.8, 1.0);
                
                cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
            },
            Scenes::Quads => {
                // Materials
                let checker_tex = Arc::new(ImageTexture::new("textures/checker.jpg"));
                let mat_tex = Material::Lambertian { tex: checker_tex };
                let checker_gen = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                let mat_gen = Material::Lambertian { tex: checker_gen };
                world.add(Shape2D::new(Vec3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Shape2DVariant::Quad, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), Shape2DVariant::Quad, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Shape2DVariant::Quad, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), Shape2DVariant::Quad, mat_tex.clone()));
                world.add(Shape2D::new(Vec3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Shape2DVariant::Quad, mat_gen.clone()));

                let earth_tex = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
                let earth_mat = Material::Lambertian { tex: earth_tex };
                world.add(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_mat));

                // Camera
                let vfov = 80.0;
                let lookfrom = Vec3::new(0.0, 0.0, 9.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                let background = Color::new(0.7, 0.8, 1.0);
                
                cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
            },
            Scenes::SimpleLight => {
                // Camera
                let vfov = 20.0;
                let lookfrom = Vec3::new(26.0, 3.0, 6.0);
                let lookat = Vec3::new(0.0, 2.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                let background = Color::new(0.0, 0.0, 0.0);
                
                // cam = Camera::new(WIDTH, HEIGHT, 100, 50, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
                cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);

                // Materials
                let checker_gen = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                let mat_gen = Material::Lambertian { tex: checker_gen };
                let difflight = Material::light_from_color(Color::new(4.0, 4.0, 4.0));

                world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, mat_gen.clone()));
                world.add(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), Shape2DVariant::Quad, difflight.clone()));
            },
            Scenes::CornellBox => {
                // Camera
                let vfov = 40.0;
                let lookfrom = Vec3::new(278.0, 278.0, -800.0);
                let lookat = Vec3::new(278.0, 278.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                let background = Color::new(0.0, 0.0, 0.0);
                
                cam = Camera::new(600, 600, 200, 50, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
                // cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);

    // auto red   = make_shared<lambertian>(color(.65, .05, .05));
    // auto white = make_shared<lambertian>(color(.73, .73, .73));
    // auto green = make_shared<lambertian>(color(.12, .45, .15));
    // auto light = make_shared<diffuse_light>(color(15, 15, 15));


                // Materials
                let red = Material::Lambertian{ tex: Material::tex_from_color(Color::new(0.65, 0.05, 0.05)) };
                let white = Material::Lambertian{ tex: Material::tex_from_color(Color::new(0.73, 0.73, 0.73)) };
                let green = Material::Lambertian{ tex: Material::tex_from_color(Color::new(0.12, 0.45, 0.15)) };
                let light = Material::light_from_color(Color::new(15.0, 15.0, 15.0));

                world.add(Shape2D::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Shape2DVariant::Quad, green.clone()));
                world.add(Shape2D::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Shape2DVariant::Quad, red.clone()));
                world.add(Shape2D::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), Shape2DVariant::Quad, light.clone()));
                world.add(Shape2D::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), Shape2DVariant::Quad, white.clone()));
                world.add(Shape2D::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), Shape2DVariant::Quad, white.clone()));
                world.add(Shape2D::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Shape2DVariant::Quad, white.clone()));
            },
            Scenes::Test => {
                // Camera
                let vfov = 80.0;
                let lookfrom = Vec3::new(0.0, 0.0, 10.0);
                let lookat = Vec3::new(0.0, 0.0, 0.0);
                let defocus_angle = 0.0;
                let focus_dist = 10.0;
                // let background = Color::new(0.7, 0.8, 1.0);
                let background = Color::new(0.0, 0.0, 0.0);
                
                cam = Camera::new(WIDTH, HEIGHT, 500, 50, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);
                // cam = Camera::new(WIDTH, HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH, vfov, lookfrom, lookat, defocus_angle, focus_dist, background);

                // Materials
                let checker_tex = Arc::new(ImageTexture::new("textures/checker.jpg"));
                let mat_tex = Material::Lambertian { tex: checker_tex };
                let checker_gen = Arc::new(CheckerTexture::from_color(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));
                let mat_gen = Material::Lambertian { tex: checker_gen };
                let earth_tex = Arc::new(ImageTexture::new("textures/earthmap.jpg"));
                let earth_mat = Material::Lambertian { tex: earth_tex };
                let difflight = Material::light_from_color(Color::new(4.0, 4.0, 4.0));

                world.add(Shape2D::new(Vec3::new(-4.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), Shape2DVariant::Disk, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(4.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), Shape2DVariant::Triangle, mat_gen.clone()));
                world.add(Shape2D::new(Vec3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), Shape2DVariant::Quad, mat_gen.clone()));
                
                world.add(Shape2D::from_center_rad_norm(Vec3::new(2.5, 0.0, 0.0), 2.0, cam.get_look_dir(), Shape2DVariant::Quad, earth_mat.clone()));
                world.add(Shape2D::from_center_rad_norm(Vec3::new(0.0, 4.0, 0.0), 2.0, &Vec3::new(0.71, 0.71, 0.0), Shape2DVariant::Quad, difflight.clone()));
                world.add(Shape2D::from_center_rad_norm(Vec3::new(7.0, 4.0, 0.0), 2.0, cam.get_look_dir(), Shape2DVariant::Disk, earth_mat.clone()));
                world.add(Sphere::new(Vec3::new(-2.5, 0.0, 0.0), 2.0, earth_mat.clone()));
            },
        }

        (BVHNode::new(world.objects), cam)
    }
}
