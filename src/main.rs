// i havent used rust in millenia btw
use std::env;
use std::fs;
use std::collections::VecDeque;
use std::any::{Any};
use std::process;
use std::io::{self, Write};

use rlua::{Lua, Result};

fn execute_lua_code(code: String) { // test function for now
    println!("lua code: {}", code);
}

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

        else if contents.chars().nth(i as usize) == Some('{') { // for lua
            let mut j = i;
            while contents.chars().nth(j as usize) != Some('}') {
                j += 1;
            }

            let code_str: String = contents.chars().skip((i + 1) as usize).take((j - i - 1) as usize).collect();
            
            execute_lua_code(code_str);
            
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

        else if contents.chars().nth(i as usize) == Some('+') {
            let one_side = &q[0].downcast_ref::<i32>().unwrap();
            let other_side = &q[1].downcast_ref::<i32>().unwrap();

            let sum = *one_side + *other_side;

            q.push_front(Box::new(sum))
        }

        else if contents.chars().nth(i as usize) == Some('-') {
            let one_side = &q[0].downcast_ref::<i32>().unwrap();
            let other_side = &q[1].downcast_ref::<i32>().unwrap();

            let sum = -*one_side + *other_side;

            q.push_front(Box::new(sum))
        }

        else if contents.chars().nth(i as usize) == Some('*') {
            let one_side = &q[0].downcast_ref::<i32>().unwrap();
            let other_side = &q[1].downcast_ref::<i32>().unwrap();

            let sum = *one_side * *other_side;

            q.push_front(Box::new(sum))
        }

        else if contents.chars().nth(i as usize) == Some('/') {
            let one_side = &q[0].downcast_ref::<i32>().unwrap();
            let other_side = &q[1].downcast_ref::<i32>().unwrap();

            let sum = *other_side / *one_side;

            q.push_front(Box::new(sum))
        }

        else if contents.chars().nth(i as usize) == Some('j') {
            let first: &Box<dyn Any> = q.get(0).unwrap();
            let second: &Box<dyn Any> = q.get(1).unwrap();

            // Convert both to strings (gpt gave me this)
            let s1 = if let Some(s) = first.downcast_ref::<String>() {
                s.clone()
            } else if let Some(n) = first.downcast_ref::<i32>() {
                n.to_string()
            } else {
                String::from("[unknown]")
            };

            let s2 = if let Some(s) = second.downcast_ref::<String>() {
                s.clone()
            } else if let Some(n) = second.downcast_ref::<i32>() {
                n.to_string()
            } else {
                String::from("[unknown]")
            };

            let combined = s2 + &s1;

            q.push_front(Box::new(combined));

        }

        else if contents.chars().nth(i as usize) == Some('r') {
            let first: &Box<dyn Any> = q.get(0).unwrap();
            let second: &Box<dyn Any> = q.get(1).unwrap();

            // Convert both to strings (gpt gave me this)
            let s1 = if let Some(s) = first.downcast_ref::<String>() {
                s.clone()
            } else if let Some(n) = first.downcast_ref::<i32>() {
                n.to_string()
            } else {
                String::from("[unknown]")
            };

            let s2 = if let Some(s) = second.downcast_ref::<String>() {
                s.clone()
            } else if let Some(n) = second.downcast_ref::<i32>() {
                n.to_string()
            } else {
                String::from("[unknown]")
            };

            let combined = s1 + &s2;

            q.push_front(Box::new(combined));

        }

        else if contents.chars().nth(i as usize) == Some('i') {
            let prompt = if let Some(s) = q.get(0).and_then(|v| v.downcast_ref::<String>()) {
                s.clone()
            } else if let Some(n) = q.get(0).and_then(|v| v.downcast_ref::<i32>()) {
                n.to_string()
            } else {
                "[unknown]".to_string()
            };

            print!("{}", prompt);
            io::stdout().flush().unwrap();

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();

            // Trim newline and push
            let input_str = user_input.trim().to_string();
            q.push_front(Box::new(input_str));
        }

        else if contents.chars().nth(i as usize) == Some('t') {
            // Get the index from q[0]
            let idx = if let Some(n) = q.get(0).and_then(|v| v.downcast_ref::<i32>()) {
                *n as usize
            } else if let Some(s) = q.get(0).and_then(|v| v.downcast_ref::<String>()) {
                s.parse::<usize>().expect("Invalid index string")
            } else {
                panic!("Expected i32 or String index at q[0]");
            };

            // Get the value at that index
            let item = q.get(idx).expect("Index out of bounds");

            // Clone it before pushing (since Box<dyn Any> isn't copyable)
            if let Some(s) = item.downcast_ref::<String>() {
                q.push_front(Box::new(s.clone()));
            } else if let Some(n) = item.downcast_ref::<i32>() {
                q.push_front(Box::new(*n));
            } else {
                panic!("Unsupported type at q[{}]", idx);
            }
        }

        else if contents.chars().nth(i as usize) == Some('>') {
            let left = q.get(0)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("I expected an integer");

            let right = q.get(1)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("I expected an integer");

            let result = *right > *left;
            if result == true {
                q.push_front(Box::new(String::from("True")));
            } else {
                q.push_front(Box::new(String::from("False")));
            }
        }

        else if contents.chars().nth(i as usize) == Some('<') {
            let left = q.get(0)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("I expected an integer");

            let right = q.get(1)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("I expected an integer");

            let result = *right < *left;
            if result == true {
                q.push_front(Box::new(String::from("True")));
            } else {
                q.push_front(Box::new(String::from("False")));
            }
        }

        else if contents.chars().nth(i as usize) == Some('=') {
            let s1 = if let Some(s) = q.get(0).and_then(|v| v.downcast_ref::<String>()) {
                s.clone()
            } else if let Some(n) = q.get(0).and_then(|v| v.downcast_ref::<i32>()) {
                n.to_string()
            } else {
                "[unknown]".to_string()
            };

            let s2 = if let Some(s) = q.get(1).and_then(|v| v.downcast_ref::<String>()) {
                s.clone()
            } else if let Some(n) = q.get(1).and_then(|v| v.downcast_ref::<i32>()) {
                n.to_string()
            } else {
                "[unknown]".to_string()
            };

            let result = *s1 == *s2;
            if result == true {
                q.push_front(Box::new(String::from("True")));
            } else {
                q.push_front(Box::new(String::from("False")));
            }
        }

        else if contents.chars().nth(i as usize) == Some(':') {
            let thing = q.get(1)
                .and_then(|v| v.downcast_ref::<String>())
                .expect("it should have been a (boolean) string");

            let number = q.get(0)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("There should be a number before");

            if thing == "False" {
                i += *number as u32;
            }
        }

        else if contents.chars().nth(i as usize) == Some(':') {
            let thing = q.get(1)
                .and_then(|v| v.downcast_ref::<String>())
                .expect("it should have been a (boolean) string");

            let number = q.get(0)
                .and_then(|v| v.downcast_ref::<i32>())
                .expect("There should be a number before");

            if thing == "True" {
                i += *number as u32;
            }
        }

        i += 1;
    }
}