use crate::types::vec3::{Vec3, Color};
use image::{Rgb, RgbImage};
mod types;

fn main() {
    let p4: Vec3 = Vec3::new(3.0, 4.0, 0.0);
    println!("{:?}; length = {}", p4, p4.length());
    println!("Execution started.");
    // Image
    let image_width: u32 = 256;
    // let ten_percent: u32 = image_width / 10;
    let image_height: u32 = 256;
    let mut img: RgbImage = RgbImage::new(image_width, image_height);

    // Render
    for x in 0..image_width {
        // if x % ten_percent == 0 {println!("In progress: {}% done", {100*x/image_width});}
        for y in 0..image_height {
            let r: f64 = x as f64 / (image_width-1) as f64;
            let g: f64 = y as f64 / (image_height-1) as f64;
            let b: f64 = 0.0;

            let color: Vec3 = <Vec3 as Color>::new(r, g, b);

            let pixel = color.rgb();
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
