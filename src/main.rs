use image::{GenericImage, GenericImageView};
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut img = image::open("examples/test01.png").unwrap();

    println!("dimensions {:?}", img.dimensions());
    println!("{:?}", img.color());

    // Obtain the image's width and height.
    let (width, height) = img.dimensions();

    let scale_factor = 0.005;
    let mut x_block = ((width as f64) * scale_factor).floor() as u32;
    let mut y_block = ((height as f64) * scale_factor * 0.5).floor() as u32;
    x_block = if x_block > 0 {x_block} else {1};
    y_block = if y_block > 0 {y_block} else {1};
    println!("Sub image size: {},{}", x_block, y_block);

    let mut out_file_buf = String::from("");

    for y in 0..height / y_block {
        for x in 0..width / x_block {
            let x_start = if x * x_block < width {
                x * x_block
            } else {
                width - x_block - 1
            };
            let y_start = if y * y_block < height {
                y * y_block
            } else {
                height - y_block - 1
            };

            let sub_img = img.sub_image(x_start, y_start, x_block, y_block);
            let mut intensity_vec = Vec::new();
            for (_, _, pix) in sub_img.pixels() {
                intensity_vec.push(pix[0] / 3 + pix[1] / 3 + pix[2] / 3);
            }

            // Average the vec
            let mut average:f32 = 0.0;
            for int in &intensity_vec{
                average += *int as f32;
            }
            average /= intensity_vec.len() as f32;
            
            out_file_buf.push(brigtness_to_ascii(average.round() as u8).unwrap());
        }
        out_file_buf.push_str(" \n")
    }

    print!("{}", out_file_buf);

    fs::remove_file("examples/out.txt").unwrap();
    let mut file = File::create("examples/out.txt").unwrap();
    file.write_all(out_file_buf.as_bytes()).unwrap();
}

// Takes a brightness from 0 to 255 and maps it to an ascii character.
fn brigtness_to_ascii(brightness: u8) -> Option<char> {
    let char_by_color = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let indx_float: f64 = (char_by_color.len() as f64 - 1.0) / (255.0) * (brightness as f64);
    println!("{} -> {} -> {}", brightness, (char_by_color.len() as f64 - 1.0) / (255.0) * (brightness as f64), char_by_color.len());
    char_by_color.chars().nth(indx_float.floor() as usize)
}
