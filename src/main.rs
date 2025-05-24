// i havent used rust in millenia btw
use std::env;
use std::fs;
use std::collections::VecDeque;
use std::any::{Any};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename: &String = &args[1];

    let contents: String = fs::read_to_string(filename)
        .expect("Should have been able to read the file");

    // skibidi toilet
    let mut q: VecDeque<Box<dyn Any>> = VecDeque::new();

    let mut i: u32 = 0;

    while i < contents.len() as u32 {
        let mut quote_count: u8 = 0;

        if contents.chars().nth(i as usize) == Some('"') { // im starting to hate rust
            quote_count += 1;
            let mut j = i;
            while quote_count != 2 {
                j += 1;
                if contents.chars().nth(j as usize) == Some('"') {
                    quote_count += 1;
                }
            }
            
            q.push_front(Box::new(contents.chars().skip((i + 1) as usize).take((j - i - 1) as usize).collect::<String>()));
            i = j;
        }

        else if contents.chars().nth(i as usize) == Some('[') { // im starting to hate rust AGAIN
            let mut j = i;
            while contents.chars().nth(j as usize) != Some(']') {
                j += 1;
            }

            let num_as_str: String = contents.chars().skip((i + 1) as usize).take((j - i - 1) as usize).collect();
            
            if let Ok(num) = num_as_str.parse::<i32>() {
                q.push_front(Box::new(num));
            } else {
                println!("Failed to parse number {}", num_as_str);
                process::exit(101);
            }
            
            
            i = j;
        }

        else if contents.chars().nth(i as usize) == Some('p') {
            let item = &q[0];
            if let Some(s) = item.downcast_ref::<String>() {
                println!("{}", s);
            } else if let Some(n) = item.downcast_ref::<i32>() {
                println!("{}", n);
            } else if let Some(f) = item.downcast_ref::<f64>() {
                println!("{}", f);
            } else {
                println!("Unknown type");
            }
        }

        i += 1;
    }
}