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
    let hex6 = Regex::new(r"(?i)[[:xdigit:]]{6}")
        .unwrap();

    loop {
        println!("Enter a hexadecimal color code (rrggbb)...");
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("stdin read_line");

        let mut found = 0;
        for m in hex6.find_iter(&line[..]) {
            found = found + 1;
            match rgb_from_str(&m.as_str()) {
                Ok(color) => {
                    let luma = luma_from_rgb(&color);
                    println!("#{} luminance = {:>5.1}%", &m.as_str(), luma*100.0);
                },
                Err(s) => {
                    println!("{} on input {}", s, &m.as_str());
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

fn rgb_from_str (s: &str) -> Result<RGB888, String> {
    if s.len() == 6 {
        Ok(RGB888 {
            r: int_from_channel(&s[0..2]),
            g: int_from_channel(&s[2..4]),
            b: int_from_channel(&s[4..6])
        })
    } else {
        Err(String::from("Unexpected string length, should have 6 hex characters"))
    }
}

fn luma_from_rgb (color: &RGB888) -> f64 {
    let r = color.r as f64;
    let g = color.g as f64;
    let b = color.b as f64;
    (0.299 * r + 0.587 * g + 0.114 * b) / 255.0
}
