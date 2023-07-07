use crate::error::AsciiError;
use anyhow::{anyhow, Result};
use image::GenericImageView;

pub struct AsciiConverter {
    image_data: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    string_rep: Option<String>,
    block_size: (u32, u32),
}

impl AsciiConverter {
    /// Create an AsciiConverter from a path to an image file
    pub fn from_path(path: String, scale_factor: f32) -> Result<Self> {
        Ok(AsciiConverter {
            image_data: image::open(path)?.to_rgb8(),
            string_rep: None,
            block_size: calculate_block_sizes(scale_factor),
        })
    }

    /// Convert the image
    pub fn convert(self) -> Result<String> {
        self.process_image()
    }

    /// Output the pre-converted string
    /// Will do the full conversion if the string has not yet been made
    pub fn get_string(self) -> Result<String> {
        if let Some(string_rep) = self.string_rep {
            Ok(string_rep)
        } else {
            self.process_image()
        }
    }

    /// Convert to the image data to a ASCII representation
    /// and a reference to an image buffer, returns the string representing that image in ASCII
    fn process_image(mut self) -> Result<String> {
        let (width, height) = self.image_data.dimensions();
        let mut out_file_buf = String::from("");

        let (x_block, y_block) = self.block_size;

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

                let sub_img = self.image_data.view(x_start, y_start, x_block, y_block);
                let mut intensity_vec = Vec::new();
                for (_, _, pix) in sub_img.pixels() {
                    intensity_vec.push(pix[0] / 3 + pix[1] / 3 + pix[2] / 3);
                }

                // Average the vec
                let average: f32 = intensity_vec
                    .iter()
                    .map(|i| *i as f32 / intensity_vec.len() as f32)
                    .sum();

                out_file_buf.push(brigtness_to_ascii(average.round() as u8)?);
            }
            out_file_buf.push_str(" \n")
        }

        self.string_rep = Some(out_file_buf.clone());
        Ok(out_file_buf)
    }
}

/// Takes a brightness from 0 to 255 and maps it to an ascii character
fn brigtness_to_ascii(brightness: u8) -> Result<char> {
    let char_by_color = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let indx_float: f32 = (char_by_color.len() as f32 - 1.0) / (255.0) * (brightness as f32);
    let out = char_by_color.chars().nth(indx_float.floor() as usize);
    match out {
        Some(char) => Ok(char),
        None => Err(anyhow!(AsciiError::CharLookupError)),
    }
}

/// Calculate the size of the "sub images", each sub image -> one ASCII character
/// Note that ASCII is taller than it is wide so ajust the scale accordingly
fn calculate_block_sizes(scale_factor: f32) -> (u32, u32) {
    let mut x_block: u32;
    let mut y_block: u32;

    let valid_scale_range = 0.0..1.0;
    if valid_scale_range.contains(&scale_factor) {
        x_block = (1.0 / scale_factor).floor() as u32;
        y_block = (2.0 / scale_factor).floor() as u32;

        // No zero size sub images
        x_block = if x_block > 0 { x_block } else { 1 };
        y_block = if y_block > 0 { y_block } else { 2 };
    } else {
        x_block = 1;
        y_block = 2;
    }
    (x_block, y_block)
}
