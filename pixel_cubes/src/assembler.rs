
extern crate image;

use image::{GenericImageView, Rgba};
use std::fmt::Binary;
use std::path::Path;
use std::fmt::Display;
use std::string;

// inspiration from Viola :)

/* System parameters. */
const INSTRUCTION_LENGTH: usize = 8;

/* Operation codes. */
type OperationCode = String;
const ADD_OP: OperationCode  = String::from("000");
const ADDI_OP: OperationCode = String::from("001");
const JMP_OP: OperationCode  = String::from("010");
const JEQ_OP: OperationCode  = String::from("011");
const SET_OP: OperationCode  = String::from("100");
const CAL_OP: OperationCode  = String::from("101");
// one more

/* Registry addresses. */
type Registry = String;
const R0: Registry = String::from("00"); // always zero
const R1: Registry = String::from("01");
const R2: Registry = String::from("10");
const R3: Registry = String::from("11");

pub fn run() {
    let code: Vec<Vec<String>> = init("test2.png");
    let mut binary: Vec<Vec<String>> = Vec::new();
    
    
    for i in 0..code.len() {
        let mut row: Vec<String> = Vec::new();

        let op = parse_op_color(code[i][0].to_string()).expect("op bruh");

        match op.0 {
            0 => {
                
            },
            1 => {

            },
            2 => {

            }
            _ => println!("bruh111")
        }
    }
}

fn init(input_path: &str) -> Vec<Vec<String>> {
    let mut code: Vec<Vec<String>> = Vec::new();

    let img = image::open(&Path::new(input_path)).expect("File not found!");

    let img_width = img.dimensions().0;
    let img_height = img.dimensions().1;
    
    let mut row_num: usize = 0;

    code.push(Vec::new());
    for p in img.pixels() {
        if to_hex(p.2) == "000000" {
            code.push(Vec::new());
            row_num += 1;
            
            print!("\n");
            continue;
        } else if to_hex(p.2) == "ffffff" {
            continue;
        }
        code[row_num].push(to_hex(p.2));

        print!("{} ", to_hex(p.2));
    }

    code
}

fn parse_op_color(color: String) -> Result<(u8, OperationCode), String> {
    // println!("hex: {}", color);
    match color.as_str() {
        "57AAA4" => Ok((0, ADD_OP)),
        "ADC9CB" => Ok((1, ADDI_OP)),
        "ECDBAB" => Ok((2, JMP_OP)),
        "F38D68" => Ok((0, JEQ_OP)),
        "E15E64" => Ok((1, SET_OP)),
        "EBD9BE" => Ok((2, CAL_OP)),
        _ => Err("invalid operation color".to_string())
    }
}

fn parse_reg_color(color: String) -> Result<(Registry), String> {
    match color.as_str() {
        "5F7D6E" => Ok((R0)),
        "AFBEB3" => Ok((R1)),
        "F4F5F4" => Ok((R2)),
        "C3A280" => Ok((R3)),
        _ => Err("invalid reg color".to_string())
    }
}

fn to_hex(rgbac: Rgba<u8>) -> String {
    format!("{:02X?}{:02X?}{:02X?}", rgbac[0], rgbac[1], rgbac[2])
}
