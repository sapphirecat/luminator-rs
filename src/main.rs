extern crate regex;

use std::io;
use regex::Regex;

#[derive(Debug)]
struct RGB888 {
    r: u8,
    g: u8,
    b: u8
}

fn main() {
    println!("Welcome to luminator-rs (with NO new features!)");
    let hex6 = Regex::new(r"(?i)([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})")
        .unwrap();

    loop {
        println!("Enter a hexadecimal color code (rrggbb)...");
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("stdin read_line");

        let mut found = 0;
        for m in hex6.captures_iter(&line[..]) {
            found = found + 1;
            match rgb_from_captures(&m) {
                Ok(color) => {
                    let luma = luma_from_rgb(&color);
                    println!("#{} luminance = {}%", &m[0], luma*100.0);
                },
                Err(s) => {
                    println!("{} on input {}", s, &m[0]);
                }
            };
        }

        if found == 0 {
            println!("No color codes found; exiting.");
            break;
        }
    }
}

fn int_from_channel(channel: &str) -> u8 {
    match u8::from_str_radix(channel, 16) {
        Ok(b) => b,
        Err(_) => 0
    }
}

fn rgb_from_captures (m: &regex::Captures) -> Result<RGB888, String> {
    if m.len() == 4 {
        Ok(RGB888 {
            r: int_from_channel(&m[1]),
            g: int_from_channel(&m[2]),
            b: int_from_channel(&m[3])
        })
    } else {
        Err(String::from("Unexpected match length, should have 3 capture groups"))
    }
}

fn luma_from_rgb (color: &RGB888) -> f64 {
    let r = color.r as f64;
    let g = color.g as f64;
    let b = color.b as f64;
    (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
}
