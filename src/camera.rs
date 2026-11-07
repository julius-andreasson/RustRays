pub mod camera {
    use image::{Rgb, RgbImage};

    use crate::types::util;
    use crate::types::vec3::{Vec3, Color};
    use crate::types::ray::Ray;
    use crate::types::interval::Interval;
    use crate::hittables::hittables::{HitRecord, Hittable, HittableList};

    pub struct Camera {
        // pub aspect_ratio: f64, // Ratio of img width over height
        pub image_width: u32,
        image_height: u32,
        center: Vec3,
        pixel00_loc: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3
    }

    impl Camera {
        pub fn render(&self, scene: &HittableList) {
            // Image
            // let ten_percent: u32 = self.image_width / 10;
            let mut img: RgbImage = RgbImage::new(self.image_width, self.image_height);

            // Render
            for x in 0..self.image_width {
                // if x % ten_percent == 0 {println!("In progress: {}% done", {100*x/self.image_width});}
                for y in 0..self.image_height {

                    let pixel_center = self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
                    let ray_dir = pixel_center - self.center;
                    let r: Ray = Ray::new(self.center, ray_dir);

                    let color = Camera::ray_color(r, &scene); // no need for ref to self

                    let pixel: Rgb<u8> = color.rgb();
                    img.put_pixel(x, y, pixel);
            } 
            }
            let res = img.save("output/test.jpg");
            if res.is_err() {
                println!("Error: {:#?}", res);
            } else {
                println!("Execution finished.");
            }
        }

        fn ray_color(r: Ray, scene: &HittableList) -> Vec3 {
            let mut rec = HitRecord::new();
            if scene.hit(&r, Interval::new(0.0, util::INF), &mut rec) {
                return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
            }

            let unit_dir = r.dir().unit_vec();
            let a = 0.5*(unit_dir.y + 1.0);
            let res =
                (1.0-a) * <Vec3 as Color>::new(1.0, 1.0, 1.0)
                    + a*<Vec3 as Color>::new(0.5, 0.7, 1.0);
            res
        }

        pub fn new(image_width: u32, image_height: u32) -> Self {
            // let image_height = (image_width as f64 / aspect_ratio) as u32;

            // Camera
            let center = Vec3::new(0.0, 0.0, 0.0);
            let focal_length = 1.0;
            let viewport_height = 2.0;
            let viewport_width = viewport_height * image_width as f64 / image_height as f64;
            
            // Calculate the vectors across the horizontal and down the vertical viewport edges. $
            let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
            let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
            
            // Calculate the horizontal and vertical delta vectors from pixel to pixel. $
            let pixel_delta_u = viewport_u / image_width as f64;
            let pixel_delta_v = viewport_v / image_height as f64;
            
            // Calculate the location of the upper left pixel. $
            let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u*0.5 - viewport_v*0.5;
            let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

            Camera { image_width, image_height, center, pixel00_loc, pixel_delta_u, pixel_delta_v }
        }
    }
}