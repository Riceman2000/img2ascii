use img2ascii::processing::AsciiConverter;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let scale_factor: f32 = 0.25;
    let in_path: String = String::from("examples/test01.jpg");
    let out_path: String = format!("{}{}", in_path, ".txt");

    let ascii_conversion = AsciiConverter::from_path(in_path, scale_factor).unwrap();
    let processed_img = ascii_conversion.convert().unwrap();

    write_output(&out_path, &processed_img).expect("Failed to write to output file");

    println!("{}", processed_img);
}

/// Write the output to a text file
fn write_output(path: &String, text: &String) -> Result<(), std::io::Error> {
    if Path::new(path).exists() {
        fs::remove_file(path)?;
    }
    File::create(path)?.write_all(text.as_bytes())?;
    Ok(())
}
