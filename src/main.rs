// i havent used rust in millenia btw
use std::env;
use std::fs;
use std::collections::VecDeque;
use std::any::{Any, TypeId};

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


        i += 1;
    }

    println!("{}", contents);
}