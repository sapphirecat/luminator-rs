extern crate regex;

use std::io;
use regex::Regex;

fn main() {
    println!("Welcome to luminator-rs (with NO new features!)");
    let hex6 = Regex::new(r"(?i)([[:xdigit:]]{2})([[:xdigit:]]{2})([[:xdigit:]]{2})")
        .unwrap();

    loop {
        println!("Enter a hexadecimal color code...");
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("stdin read_line");

        let mut found = 0;
        for m in hex6.captures_iter(&line[..]) {
            found = found + 1;

            let r = value_of_channel(&m[1]);
            let g = value_of_channel(&m[2]);
            let b = value_of_channel(&m[3]);

            let luma = 0.299 * r + 0.587 * g + 0.114 * b;
            println!("#{} luminance = {}", &m[0], luma);
        }

        if found == 0 {
            println!("No color codes found; exiting.");
            break;
        }
    }
}

fn value_of_channel(channel: &str) -> f64 {
    match u8::from_str_radix(channel, 16) {
        Ok(b) => b as f64,
        Err(_) => 0.0
    }
}
