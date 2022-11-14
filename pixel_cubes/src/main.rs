extern crate image;

use std::path::Path;
use std::{f32, fs::File};

use image::{GenericImageView, Rgba};
use image::Pixel;
use image::Pixels;



fn main() {
    let code: Vec<u8> = init("test.png");
    
}

fn init(input_path: &str) -> Vec<u8> {
    let img = image::open(&Path::new(input_path)).expect("File not found!");

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    let mut code: Vec<u8> = Vec::new();

    for p in img.pixels() {
        code.push(p.2[0]);
        code.push(p.2[1]);
        code.push(p.2[2]);
        code.push(p.2[3]);
        println!("{:b}\n{:b}\n{:b}\n{:b}", p.2[0], p.2[1], p.2[2], p.2[3]);
    }

    code
}
