use image::GenericImageView;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let scale_factor: f32 = 1.0;
    let in_path: String = "examples/test03.png".to_owned();
    let out_path: String = format!("{}{}", in_path, ".txt");

    let img = image::open(in_path);
    let img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = match img {
        Ok(image) => image.to_rgb8(),
        Err(error) => {
            println!("Error opening image: {}", error);
            return;
        }
    };

    println!("dimensions: {:?}", img.dimensions());

    let (x_block, y_block) = calculate_block_sizes(scale_factor);
    println!("Sub image size: {},{}", x_block, y_block);

    let processed_img = process_image(x_block, y_block, &img);
    let processed_img = match processed_img {
        Ok(text) => text,
        Err(error) => {
            println!("Failed with error: {}", error);
            return;
        }
    };
    match write_output(&out_path, &processed_img) {
        Ok(_) => {}
        Err(error) => {
            println!("Failed to write output file: {}", error);
        }
    };

    println!("{}", processed_img);
}

// Calculate the size of the sub images to average for every ASCII character
fn calculate_block_sizes(scale_factor: f32) -> (u32, u32) {
    // Calculate the size of the sub images to average, each sub image -> one character
    // Note that ASCII is taller than it is wide so ajust the scale accordingly
    let mut x_block: u32;
    let mut y_block: u32;
    if (0.0..1.0).contains(&scale_factor) {
        // Valid scale factor -> calculate block sizes
        x_block = (1.0 / scale_factor).floor() as u32;
        y_block = (2.0 / scale_factor).floor() as u32;
        // Make sure even really small blocks still look right/work at all
        x_block = if x_block > 0 { x_block } else { 1 };
        y_block = if y_block > 0 { y_block } else { 2 }; // Height vs width of ASCII
    } else {
        // Invalid scale factor -> manually assign
        x_block = 1;
        y_block = 2;
    }
    (x_block, y_block)
}

// Write the output to a text file
fn write_output(path: &String, text: &String) -> Result<(), std::io::Error> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    File::create(path)?.write_all(text.as_bytes())?;
    Ok(())
}

// Takes a brightness from 0 to 255 and maps it to an ascii character.
fn brigtness_to_ascii(brightness: u8) -> Result<char, &'static str> {
    let char_by_color = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let indx_float: f32 = (char_by_color.len() as f32 - 1.0) / (255.0) * (brightness as f32);
    let out = char_by_color.chars().nth(indx_float.floor() as usize);
    match out {
        Some(char) => Ok(char),
        None => Err("Failed to convert brightness to ASCII."),
    }
}

// Takes the block sizes (defines how many pixels are averaged for one ASCII character)
// and a reference to an image buffer, returns the string representing that image in ASCII
fn process_image(
    x_block: u32,
    y_block: u32,
    img: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) -> Result<String, &str> {
    let (width, height) = img.dimensions();
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

            let sub_img = img.view(x_start, y_start, x_block, y_block);
            let mut intensity_vec = Vec::new();
            for (_, _, pix) in sub_img.pixels() {
                intensity_vec.push(pix[0] / 3 + pix[1] / 3 + pix[2] / 3);
            }

            // Average the vec
            let mut average: f32 = 0.0;
            for int in &intensity_vec {
                average += *int as f32;
            }
            average /= intensity_vec.len() as f32;

            out_file_buf.push(brigtness_to_ascii(average.round() as u8)?);
        }
        out_file_buf.push_str(" \n")
    }

    Ok(out_file_buf)
}
