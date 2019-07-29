use std::io::{self};

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line!");

    let nums : Vec<i32> = input.split_whitespace().map(|value| value.parse().unwrap()).collect();

    let max_value = nums[0] + nums[1];

    let mut values : Vec<i32> = vec![0; max_value as usize];
    let mut max : i32 = 0;

    for first_value in 1..nums[0]+1{
        for second_value in 1..nums[1]+1{
            
            let temp : i32 = first_value + second_value;
            values[(temp-1) as usize] += 1;

            if values[(temp-1) as usize] > max {
                max = values[(temp-1) as usize];
            }
        }
    }
    print_values(values, &max);
}


fn print_values(values: Vec<i32>, max: &i32){
    for (i, item) in values.iter().enumerate(){
        if item == max {
            println!("{}", (i+1));
        }
    }
}
