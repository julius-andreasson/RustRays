pub mod texture {
    use std::{process::exit, sync::Arc};
    use image::ImageReader;

    use crate::types::{interval::Interval, vec3::{Color, Vec3}};

    pub type T = dyn Texture + Sync + Send;

    pub trait Texture {
        fn value(&self, u: f64, v: f64, p: &Vec3) -> Color;
    }

    pub struct SolidColor {
        albedo: Color,
    }

    impl SolidColor {
        pub fn from_color(albedo: Color) -> Self {
            Self { albedo }
        }

        pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
            Self { albedo: Color::new(r, g, b) }
        }
    }

    impl Texture for SolidColor { 
        fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
            self.albedo.clone()
        }
    }

    // Checker
    pub struct CheckerTexture {
        inv_scale: f64,
        even: Arc<T>,
        odd: Arc<T>,
    }

    impl CheckerTexture {
        pub fn from_texture(scale: f64, even: Arc<T>, odd: Arc<T>) -> Self {
            Self { inv_scale: 1.0/scale, even, odd }
        }
        pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
            
            let even = Arc::new(SolidColor::from_color(c1));
            let odd = Arc::new(SolidColor::from_color(c2));
            Self { inv_scale: 1.0/scale, even, odd }
        }
    }

    impl Texture for CheckerTexture {
        fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
            let x_integer = (self.inv_scale * p.x).floor() as i64;
            let y_integer = (self.inv_scale * p.y).floor() as i64;
            let z_integer = (self.inv_scale * p.z).floor() as i64;

            let is_even = (x_integer+y_integer+z_integer)%2 == 0;
            // let is_even = ((p.x+p.y+p.z)*self.inv_scale).floor() as i64 % 2 == 0;
            if is_even {
                return self.even.value(u, v, p);
            } else {
                return self.odd.value(u, v, p);
            }
        }
    }

    pub struct ImageTexture {
        img: image::RgbImage,
    }

    impl ImageTexture {
        pub fn new(filename: &str) -> Self {
            let read = ImageReader::open(filename);
            let img = read.unwrap().decode().unwrap().into_rgb8();
            Self { img }
        }
    }

    impl Texture for ImageTexture {
        fn value(&self, u: f64, v: f64, p: &Vec3) -> Color {
            // If we have no texture data, then return solid cyan as a debugging aid. $
            if self.img.height() <= 0 {return Color::new(0.0, 1.0, 1.0)};

            // Clamp input texture coordinates to [0,1] x [1,0] $
            let u = Interval::new(0.0, 1.0).clamp(u);
            // Flip V to image coordinates $
            let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); 

            let i = (u * self.img.width() as f64) as u32;
            let j = (v * self.img.height() as f64) as u32;
            let pixel: [u8; 3] = self.img.get_pixel(i, j).0;

            let color_scale = 1.0/255.0;
            return Color::new(color_scale*pixel[0] as f64, color_scale*pixel[1] as f64, color_scale*pixel[2] as f64);
        }
    }
//     class image_texture : public texture {
//   public:
//     image_texture(const char* filename) : image(filename) {}

}