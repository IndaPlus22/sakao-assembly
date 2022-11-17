
extern crate image;

use image::{GenericImageView, Rgba};
use std::fmt::Binary;
use std::path::Path;
use std::fmt::Display;
use std::string;
use std::u64;

// inspiration from Viola :)

/* System parameters. */
const INSTRUCTION_LENGTH: usize = 8;

/* Operation codes. */
const ADD_OP: &str  = "000";
const ADDI_OP: &str = "001";
const JMP_OP: &str  = "010";
const JEQ_OP: &str  = "011";
const SET_OP: &str  = "100";
const CAL_OP: &str  = "101";
// one more

/* &str addresses. */
const R0: &str = "00"; // always zero
const R1: &str = "01";
const R2: &str = "10";
const R3: &str = "11";

pub fn run(input_path: &str) {
    let code: Vec<Vec<String>> = init(input_path);
    let mut binary: Vec<String> = Vec::new();
    
    for i in 0..code.len() {
        let op = parse_op_color(code[i][0].to_string()).expect("op bruh");

        binary.push(op.1.to_string());
        match op.0 {
            0 => { // arithmetic op
                binary[i].push_str(parse_reg_color(code[i][1].to_string()).expect("reg fail").as_str());
                binary[i].push_str(parse_reg_color(code[i][2].to_string()).expect("reg fail").as_str());
            },
            1 => { // imm op
                binary[i].push_str(parse_reg_color(code[i][1].to_string()).expect("reg fail").as_str());
                binary[i].push_str(parse_imm_color(code[i][2].to_string()).expect("imm fail").as_str());
            },
            2 => { // jmp op
                binary[i].push_str(parse_adress_color(code[i][1].to_string()).expect("reg fail").as_str());
            }
            _ => println!("bruh111")
        }
    }

    for i in 0..code.len() {
        println!("{}: {}", i, binary[i]);
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
    code = remove_empty_vecs(code);
    code
}

fn remove_empty_vecs(vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut tmp = Vec::new();
    
    for i in 0..vec.len() {
        if vec[i].len() != 0 {
            tmp.push(vec[i].clone());
        }
    }
    tmp
}

fn parse_op_color(color: String) -> Result<(u8, String), String> {
    // println!("hex: {}", color);
    match color.as_str() {
        "57AAA4" => Ok((0, ADD_OP.to_string())),
        "ADC9CB" => Ok((1, ADDI_OP.to_string())),
        "ECDBAB" => Ok((2, JMP_OP.to_string())),
        "F38D68" => Ok((0, JEQ_OP.to_string())),
        "E15E64" => Ok((1, SET_OP.to_string())),
        "EBD9BE" => Ok((2, CAL_OP.to_string())),
        _ => Err("invalid operation color".to_string())
    }
}

fn parse_reg_color(color: String) -> Result<String, String> {
    match color.as_str() {
        "5F7D6E" => Ok(R0.to_string()),
        "AFBEB3" => Ok(R1.to_string()),
        "F4F5F4" => Ok(R2.to_string()),
        "C3A280" => Ok(R3.to_string()),
        _ => Err("invalid reg color".to_string())
    }
}


fn parse_imm_color(color: String) -> Result<String, String> {
    let num = parse_num_color(color);
    if num < 1 || num > 8 {
        Err("invalid imm color".to_string())
    } else {
        Ok(format!("{:03b}", num))
    }
}

fn parse_num_color(color: String) -> u64 {
    let mut tmp: u64 = u64::from_str_radix(color.as_str(), 16).unwrap();
    let cerise: u64 = 0xDE3163; 

    tmp -= cerise;
    tmp / 10
}

// TODO:
fn parse_adress_color(color: String) -> Result<String, String> {
    let num = parse_num_color(color);
    if num < 0 || num > 2^5 {
        Ok(format!("{:05X?}", num))
    } else {
        Err("invalid adress color".to_string())
    }
}

fn to_hex(rgbac: Rgba<u8>) -> String {
    format!("{:02X?}{:02X?}{:02X?}", rgbac[0], rgbac[1], rgbac[2])
}