pub mod texture {
    use std::sync::Arc;
    use image::ImageReader;

    use crate::types::{interval::Interval, vec3::{Color, Vec3}};

    pub type Tex = dyn Texture + Sync + Send;

    // let checker = Arc::new(CheckerTexture::from_color
    pub static ERROR_TEXTURE: ErrorTexture = ErrorTexture::new();

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

    // ErrorTexture
    pub struct ErrorTexture(f64, Color, Color);

    impl ErrorTexture {
        pub const fn new() -> Self {
            let even = Color { x: 0.0, y: 0.0, z: 0.0 };
            let odd = Color { x: 57.0, y: 255.0, z: 20.0 };
            Self { 0: 1.2, 1: even, 2: odd }
        }
    }

    impl Texture for ErrorTexture {
        fn value(&self, _u: f64,_v: f64, p: &Vec3) -> Color {
            let x_integer = (self.0 * p.x).floor() as i64;
            let y_integer = (self.0 * p.y).floor() as i64;
            let z_integer = (self.0 * p.z).floor() as i64;

            let is_even = (x_integer+y_integer+z_integer)%2 == 0;
            // let is_even = ((p.x+p.y+p.z)*self.0).floor() as i64 % 2 == 0;
            if is_even {
                return self.1.clone();
            } else {
                return self.2.clone();
            }
        }
    }

    // Checker
    pub struct CheckerTexture {
        inv_scale: f64,
        even: Arc<Tex>,
        odd: Arc<Tex>,
    }

    impl CheckerTexture {
        pub fn from_texture(scale: f64, even: Arc<Tex>, odd: Arc<Tex>) -> Self {
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

            let pixel: [u8; 3];
            let (x, y) = ((u * self.img.width() as f64) as i32, (v * self.img.height() as f64) as i32);
            if (0 < x && (x as u32) < self.img.width()) && (0 < y && (y as u32) < self.img.height()) {
                // safe
                let (safe_x, safe_y) = (x as u32, y as u32);
                pixel = self.img.get_pixel(safe_x, safe_y).0;
            } else {
                // Error texture
                let err_tex = ERROR_TEXTURE.value(u, v, p);
                pixel = [err_tex.x as u8, err_tex.y as u8, err_tex.z as u8];
            }

            let color_scale = 1.0/255.0;
            return Color::new(color_scale*pixel[0] as f64, color_scale*pixel[1] as f64, color_scale*pixel[2] as f64);
        }
    }
//     class image_texture : public texture {
//   public:
//     image_texture(const char* filename) : image(filename) {}

}