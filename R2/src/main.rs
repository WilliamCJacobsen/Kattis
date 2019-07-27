use std::io;
use std::io::prelude::*;

fn main(){
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
        
    let nums: Vec<i32> = input_text.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
            
    let value : i32 = nums[1] * 2 - nums[0];
    
    println!("{}", value);
}