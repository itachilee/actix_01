use image::{ImageBuffer, Rgba,RgbaImage,Luma,DynamicImage};
use rand;
use actix_web::{get, HttpResponse,Responder};
use std::io::{Cursor};
use std::time::{Instant};
use num_complex::{Complex};

fn generate_image(width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = ImageBuffer::new(width, height);

    // Iterate through each pixel and set it to a random color
    for (_, _, pixel) in img.enumerate_pixels_mut() {
        *pixel = image::Rgba([
            rand::random::<u8>(), // Red
            rand::random::<u8>(), // Green
            rand::random::<u8>(), // Blue
            255, // Alpha
        ]);
    }


   

    img
}



fn generate_image_rgba() -> RgbaImage {

    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = RgbaImage::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.5 * x as f32) as u8;
        let b = (0.5 * y as f32) as u8;
        *pixel = image::Rgba([r, 0, b,255]);
    }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            // let c = num_complex::Complex::new(-0.4, 0.6);
            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgba(data) = *pixel;
            *pixel = image::Rgba([data[0], i as u8, data[2],255]);
        }
    }


    imgbuf
}


enum Fractal{
    RGBA,
    LUMA
}

fn newton_fractal(c: Complex<f64>, max_iter: usize) -> u8 {
    let mut z = Complex::new(0.0, 0.0);
    for n in 0..max_iter {
        // z.abs_sqr > 4.0
        if z.norm() > 2.0 {
            return n as u8;
        }
        z = z * z + c;
    }
    return 0;
}


fn newton_fractal_rgba(c: Complex<f64>, max_iter: usize) -> Option<Rgba<u8>> {
    let mut z = Complex::new(0.0, 0.0);
    for n in 0..max_iter {
        if z.norm() > 2.0 {
            let mut color = Rgba([0, 0, 0, 255]);
            // Adjust color based on iteration count
            let hue = (n % 256) as f64 / 256.0;
            color.0[0] = hue_to_rgb(hue, 1.0, 0.0);
            color.0[1] = hue_to_rgb(hue, 0.0, 1.0);
            color.0[2] = hue_to_rgb(hue, 0.5, 0.0);
            return Some(color);
        }
        z = z * z + c;
    }
    None
}

fn hue_to_rgb(hue: f64, r: f64, g: f64) -> u8 {
    let q = if hue < 0.0 { hue + 1.0 } else if hue >= 1.0 { hue - 1.0 } else { hue };
    let k = (q * 6.0).floor();
    let f = q * 6.0 - k;
    let t = 1.0 - f;
    let v = r * f + g * t;
    let  intensity = (255.0 * v) as u8;
   
    intensity
}

fn create_newton_fractal(width: u32, height: u32,max_iter: usize,ftype: Fractal)-> ImageBuffer<Luma<u8>, Vec<u8>>{

    match ftype {
        Fractal::RGBA=>{
            let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
            for x in 0..width {
                for y in 0..height {
                    let real = (x as f64 / (width - 1) as f64) * 3.0 - 2.0;
                    let imag = (y as f64 / (height - 1) as f64) * 2.0 - 1.0;
                    let c = Complex::new(real, imag);
                    let pixel_color = newton_fractal_rgba(c, max_iter);
        
                    if let Some(color) = pixel_color {
                        image.put_pixel(x, y, color);
                    }
                }
            }
            DynamicImage::ImageRgba8(image).into_luma8()
      
        },
        Fractal::LUMA=>{
            let mut image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
         
            for x in 0..width {
                for y in 0..height {
                    let real = (x as f64 / (width - 1) as f64) * 3.0 - 2.0;
                    let imag = (y as f64 / (height - 1) as f64) * 2.0 - 1.0;
                    let c = num_complex::Complex::new(real, imag);
                    let iter = newton_fractal(c, max_iter);
                    image.put_pixel(x, y, Luma([iter]));
                }
            }
            image
        }
    }

}




#[get("/imgs")]
pub async fn generate_image_handler() -> impl Responder {


    let start = Instant::now();
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Generate the image
    // let img = generate_image(WIDTH, HEIGHT);
    // let img = generate_image_rgba();
    let img =create_newton_fractal(WIDTH,HEIGHT,20,Fractal::RGBA);

    // Convert the image to PNG format (or any other desired format)
    let mut buf =Cursor::new( Vec::new());
    img.write_to(&mut buf, image::ImageOutputFormat::Png)
        .expect("Failed to write image to buffer");

    let duration = start.elapsed();
    println!("Time elapsed in generate_image_rgba() is: {:?}", duration);
    // Return the image data as a response
    HttpResponse::Ok()
        .content_type("image/png")
        .body(buf.into_inner())
}


#[cfg(test)]
mod complex_test{
    /// z1=a+bi，z2=c+di
    /// 则它们的和是 (a+bi)+(c+di)=(a+c)+(b+d)i。
    /// 两个复数的和依然是复数，它的实部是原来两个复数实部的和，它的虚部是原来两个虚部的和。
    /// 复数的加法满足交换律和结合律，
    /// 即对任意复数z1，z2，z3，有： z1+z2=z2+z1；(z1+z2)+z3=z1+(z2+z3)。
    #[test]
    fn test_complex_add(){

        let a =num_complex::Complex::new(-0.4, 0.6);
        let b =num_complex::Complex::new(-0.4, 0.6);

        let c =a+b;
        debug_assert_eq!(-0.8,c.re);
        debug_assert_eq!(1.2,c.im);
    }

    #[test]
    fn test_complex_subtract(){

        let a =num_complex::Complex::new(-0.4, 0.6);
        let b =num_complex::Complex::new(-0.4, 0.6);

        let c =a-b;
        debug_assert_eq!(0f32,c.re);
        debug_assert_eq!(0f32,c.im);
    }

    /// 设z1=a+bi，z2=c+di(a、b、c、d∈R)是任意两个复数，那么它们的积(a+bi)(c+di)=(ac-bd)+(bc+ad)i。
    /// 把两个复数相乘，类似两个多项式相乘，展开得: ac+adi+bci+bdi2，因为i2=-1，所以结果是(ac－bd)+(bc+ad)i 。两个复数的积仍然是一个复数。
    #[test]
    fn test_complex_multiply(){
        let a =num_complex::Complex::new(-0.4, 0.6);
        let b =num_complex::Complex::new(-0.4, 0.6);

        let c: num_complex::Complex<f32> =a * b;

        debug_assert_eq!((-0.4*-0.4) -(0.6*0.6),c.re);
        debug_assert_eq!( (0.6 * -0.4) + (-0.4 * 0.6) ,c.im);
    }

    /// 设z1=a+bi，z2=c+di(a、b、c、d∈R)是任意两个复数
    /// 则(a+bi)/(c+di)=(ac+bd)/(c2+d2) +((bc-ad)/(c2+d2))i
    /// 
    #[test]
    fn test_complex_devide(){
        let a =num_complex::Complex::new(-0.4, 0.6);
        let b =num_complex::Complex::new(-0.4, 0.6);

        let c: num_complex::Complex<f32> =a / b;


        println!("real is {}",(0.6 * -0.4)- (-0.4 * 0.6) );
        debug_assert_eq!( ((-0.4 * -0.4) +(0.6*0.6)) /((-0.4*-0.4) +(0.6 * 0.6)) ,c.re);

        debug_assert_eq!( ((0.6 * -0.4)- (-0.4 * 0.6)) /((-0.4*-0.4) +(0.6 * 0.6))  ,c.im);

    }

    #[test]
    fn number_should_be_zero(){

        let a =0f32;
        let b =32f32;
        let c =a/b;
        debug_assert_eq!(0f32,c);

    }

}

