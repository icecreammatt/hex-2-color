use colored::*;
use hex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut r = String::from("FF");
    let mut g = String::from("FF");
    let mut b = String::from("FF");

    /* TODO
    - Add length check 6 even for hex
    - Add range check for 0-9A-F
    */

    if args.len() < 2 {
        println!("Usage: hex-2-color <FFFFFF or (0-255) (0-255) (0-255)>")
    }

    if args.len() == 2 {
        let hex_value = &args[1].to_uppercase();
        println!("HEX: 0x{}", &hex_value);
        r = String::from(&hex_value[0..2]);
        g = String::from(&hex_value[2..4]);
        b = String::from(&hex_value[4..6]);
    }

    if args.len() == 3 {
        println!("Usage: hex-2-color <FFFFFF or (0-255) (0-255) (0-255)>")
    }

    if args.len() == 4 {
        println!("integers RGB");
        r = String::from(&args[1]);
        g = String::from(&args[2]);
        b = String::from(&args[3]);
    }

    if args.len() > 3 {
        println!("Usage: hex-2-color <FFFFFF or (0-255) (0-255) (0-255)>")
    }

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

    let red = hex::decode(&r).unwrap()[0];
    let green = hex::decode(&g).unwrap()[0];
    let blue = hex::decode(&b).unwrap()[0];

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
