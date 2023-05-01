use colored::*;
use hex;
use std::env;
use std::process;

// Conversion
// #4C566A
// 4    C    5    6    6    A
// 0100 1100 0101 0110 0110 1010
// 01001100  01010110  01101010
// 64+8+4    64+16+4+2 64+32+8+2
// 76        86        106
// let r = "4C";
// let g = "56";
// let b = "6A";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut r = String::from("FF");
    let mut g = String::from("FF");
    let mut b = String::from("FF");

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    /* TODO
    - Add length check 6 even for hex
    - Add range check for 0-9A-F
    */

    let valid_arg_len = [2usize, 4usize];

    if !valid_arg_len.contains(&args.len()) {
        println!("Usage: hex-2-color <FFFFFF or (0-255) (0-255) (0-255)>");
        process::exit(1);
    }

    if args.len() == 2 {
        let hex_value = &args[1].to_uppercase();
        r = String::from(&hex_value[0..2]);
        g = String::from(&hex_value[2..4]);
        b = String::from(&hex_value[4..6]);

        red = hex::decode(&r).unwrap()[0];
        green = hex::decode(&g).unwrap()[0];
        blue = hex::decode(&b).unwrap()[0];
    }

    if args.len() == 4 {
        red = (&args[1]).parse::<u8>().unwrap();
        green = (&args[2]).parse::<u8>().unwrap();
        blue = (&args[3]).parse::<u8>().unwrap();
    }

    println!(
        "\n{}\n{}HEX #{}{}{}{}\n{}\n{}R({}) G({}) B({}){}\n{}\n{}\n{}\n{}\n{}\n{}\n",
        "████████████████████████".truecolor(red, green, blue),
        "██".truecolor(red, green, blue),
        &r,
        &g,
        &b,
        "███████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "██".truecolor(red, green, blue),
        format!("{:>3}", red).to_string(),
        format!("{:>3}", green).to_string(),
        format!("{:>3}", blue).to_string(),
        "██".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue),
        "████████████████████████".truecolor(red, green, blue)
    );
}
