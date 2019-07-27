use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let mut numbers : Vec<Vec<i32>> = Vec::new();
    let mut path : Vec<i32> = Vec::new();

    for line in input.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums[0] == -1{
            break;
        }
        numbers.push(nums);
    }

    let mut kitten_pos : i32 = numbers.first().unwrap().to_vec()[0];

    path.push(kitten_pos);

    let mut jumped : bool = false;
    loop {
        jumped = false;
        // can try to make it fuctional. because it is sexy.
        for vector in numbers.iter().skip(1){
            let next_branch : &i32 = vector.first().unwrap();

                for x in vector.iter().skip(1) {
                    if kitten_pos == *x {
                        kitten_pos = next_branch.clone();
                        jumped = true;
                        path.push(kitten_pos)
                }
            }
        }

        if jumped == false {
            break;
        }
    }

    let mut path_string : String = String::new();

    for x in path.iter(){
        path_string.push_str(&x.to_string()[..]);
        path_string.push_str(" ");
    }
    path_string.pop();

    println!("{}",path_string);
}