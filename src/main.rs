use image::{Rgb, RgbImage};
mod types;

fn main() {
    println!("Execution started.");
    // Image
    let image_width: u32 = 256;
    let image_height: u32 = 256;
    let mut img: RgbImage = RgbImage::new(image_width, image_height);

    // Render
    for x in 0..image_width {
        for y in 0..image_height {
            let r: f64 = x as f64 / (image_width-1) as f64;
            let g: f64 = y as f64 / (image_height-1) as f64;
            let b: f64 = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            // std::cout << ir << ' ' << ig << ' ' << ib << '\n';
            let pixel = Rgb([ir, ig, ib]);
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
