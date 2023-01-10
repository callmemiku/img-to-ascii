use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::structs::Args;
mod structs;
use std::path::PathBuf;
use image::{DynamicImage, GenericImageView};
use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() {
    let args = Args::parse_args();
    let path = PathBuf::from(args.path);
    let image = open_image(path).expect("Couldn't open image!");
    let art = convert_to_ascii(image, args.resolution as u32);
    let output = if &args.output == "" { PathBuf::from("output.txt") } else { PathBuf::from(args.output) };
    write_to_file(output, art).expect("Couldn't write!");
    println!("D O N E !");
}

fn open_image(path: PathBuf) -> Result<DynamicImage, Box<dyn Error>> {
    let file = ImageReader::open(path)?.decode();
    let image = match file {
        Ok(file) => { file },
        Err(error) => {
            return Err(error.into())
        }
    };
    Ok(image)
}

fn convert_to_ascii(img: DynamicImage, resolution: u32) -> String {
    let mut art = String::new();
    let symbols = ["@", "#", "O", "0", "*", "a", "o", ",", "."];
    let smaller = img.resize(
        img.width() / resolution,
        img.height() / resolution,
        FilterType::Nearest
    );

    let mut stop = 0;

    println!("Original size: {:?}   Reduced: {:?}", img.dimensions(), smaller.dimensions());

    for pixel in smaller.pixels() {
        if stop != pixel.1 {
            art.push_str("\n");
            stop = pixel.1
        }
        let data = pixel.2;
        let brightness: f64 = ((data[0] as u64 + data[1] as u64 + data[2] as u64) / 3) as f64;
        let character_position = ((brightness/255.0) * (symbols.len() - 1) as f64 ).round() as usize;
        art.push_str(symbols[character_position])
    }

    art
}

fn write_to_file(path: PathBuf, art: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path).expect("Couldn't create the file!");
    file.write_all(art.as_bytes())?;
    Ok(())
}

