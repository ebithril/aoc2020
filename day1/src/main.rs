use std::fs::File;
use std::io::prelude::*;

fn part1(numbers: &Vec<i32>)
{
    for (i, number) in numbers.iter().enumerate() {
        for j in i+1..numbers.len() {
            let other_number = numbers[j];
            if number + other_number != 2020 {
                continue;
            }

            println!("Part1: {}", number * other_number);
            return;
        }
    }
}

fn part2(numbers: &Vec<i32>)
{
    for (i, number) in numbers.iter().enumerate() {
        for j in i+1..numbers.len() {
            let other_number = numbers[j];
            for k in j+1..numbers.len() {
                let third_number = numbers[k];
                if number + other_number + third_number != 2020 {
                    continue;
                }

                println!("Part2: {}", number * other_number * third_number);
                return;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input1.txt")?;

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

    part1(&numbers);
    part2(&numbers);

    Ok(())
}
