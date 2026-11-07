pub mod camera {
    use image::{Rgb, RgbImage};

    use crate::types::util::{self, degrees_to_radians, rnd};
    use crate::types::vec3::{Vec3, Color};
    use crate::types::ray::Ray;
    use crate::types::interval::Interval;
    use crate::hittables::hittables::{self, HitRecord, Hittable, HittableList, Material};

    pub struct Camera {
        // pub aspect_ratio: f64, // Ratio of img width over height
        image_width: u32,
        image_height: u32,
        center: Vec3,
        pixel00_loc: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        samples_per_pixel: u32,
        max_depth: i32,
        defocus_disk_u: Vec3,
        defocus_disk_v: Vec3,
        defocus_angle: f64
    }

    impl Camera {
        pub fn render(&self, scene: &HittableList) -> Result<(), image::ImageError> {
            // Image
            let ten_percent: u32 = self.image_width / 10;
            let mut img: RgbImage = RgbImage::new(self.image_width, self.image_height);

            // Render
            for x in 0..self.image_width {
                if x % ten_percent == 0 {println!("In progress: {}% done", {100*x/self.image_width});}
                for y in 0..self.image_height {

                    let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                    for _sample in 0..self.samples_per_pixel {
                        let r = self.get_ray(x, y);
                        pixel_color += Camera::ray_color(r, self.max_depth, &scene);
                    }
                    let color = pixel_color / self.samples_per_pixel as f64;

                    let pixel: Rgb<u8> = color.rgb();
                    img.put_pixel(x, y, pixel);
            } 
            }
            return img.save("output/test.jpg");
        }

        fn ray_color(r: Ray, remaining_depth: i32, scene: &HittableList) -> Vec3 {
            if remaining_depth <= 0 { return Vec3::new(0.0, 0.0, 0.0)};
            
            let mut rec = HitRecord::new();
            if scene.hit(&r, Interval::new(0.001, util::INF), &mut rec) {
                let mut scattered = Ray::default();
                let mut attenuation = Vec3::default();
                if hittables::scatter(&r, &rec, &mut attenuation, &mut scattered, rec.mat) {
                    return attenuation * Camera::ray_color(scattered, remaining_depth - 1, &scene);
                }
                return Vec3::default();

            } else {
                let unit_dir = r.dir().unit_vec();
                let a = 0.5*(unit_dir.y + 1.0);
                let res =
                    (1.0-a) * <Vec3 as Color>::new(1.0, 1.0, 1.0)
                        + a*<Vec3 as Color>::new(0.5, 0.7, 1.0);
                return res;
            }
        }

        pub fn new(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: i32) -> Self {
            // Camera config
            let vfov = 20.0;
            let lookfrom = Vec3::new(13.0, 2.0, 3.0);
            let lookat = Vec3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);

            let defocus_angle = 0.6;
            let focus_dist = 10.0;

            let center = lookfrom;
            // let focal_length = (lookfrom - lookat).length();
            let theta = degrees_to_radians(vfov);
            let h = (theta/2.0).tan();
            let viewport_height = 2.0 * h * focus_dist;
            let viewport_width = viewport_height * image_width as f64 / image_height as f64;
            
            let w = (lookfrom - lookat).unit_vec();
            let u = vup.cross(w).unit_vec();
            let v = w.cross(u);
            
            // Calculate the vectors across the horizontal and down the vertical viewport edges. $
            let viewport_u = viewport_width * u;
            let viewport_v = viewport_height * -v;
            
            // Calculate the horizontal and vertical delta vectors from pixel to pixel. $
            let pixel_delta_u = viewport_u / image_width as f64;
            let pixel_delta_v = viewport_v / image_height as f64;
            
            // Calculate the location of the upper left pixel. $
            let viewport_upper_left = center - (focus_dist * w) - viewport_u*0.5 - viewport_v*0.5;
            let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

            let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
            let defocus_disk_u = u * defocus_radius;
            let defocus_disk_v = v * defocus_radius;

            Camera { image_width, image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v, samples_per_pixel, max_depth, defocus_disk_u, defocus_disk_v, defocus_angle }
        }
        
        fn get_ray(&self, x: u32, y: u32) -> Ray {
            let offset = sample_square();
            let pixel_sample = self.pixel00_loc
                                    + (x as f64 + offset.x) * self.pixel_delta_u
                                    + (y as f64 + offset.y) * self.pixel_delta_v;

            let ray_orig = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
            let ray_dir = pixel_sample - ray_orig;
            
            let r: Ray = Ray::new(ray_orig, ray_dir);
            r
        }

        fn defocus_disk_sample(&self) -> Vec3 {
            let p = Vec3::random_in_unit_disc();
            let res = self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v);
            res
        }
    }
    
    fn sample_square() -> Vec3 {
        Vec3::new(rnd() - 0.5, rnd() - 0.5, 0.0)
    }
}