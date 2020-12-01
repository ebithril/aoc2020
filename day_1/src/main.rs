use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split("\n");

    let mut numbers: Vec<i32>  = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }
        let number: i32 = line.parse().unwrap();
        numbers.push(number);
    }

    for (i, number) in numbers.iter().enumerate() {
        for j in i+1..numbers.len() {
            let other_number = numbers[j];
            for k in j+1..numbers.len() {
                let third_number = numbers[k];
                if number + other_number + third_number != 2020 {
                    continue;
                }

                println!("{}", number * other_number * third_number);
            }
        }
    }

    Ok(())
}
