use std::io::{self, BufRead};

fn main(){
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            for x in 1..(i+1)  {
                println!("{} Abracadabra", x);
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}