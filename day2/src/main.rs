use std::fs::File;
use std::io::prelude::*;

fn parse_password(password: &String) -> (usize, usize, String, String) {
    let mut pass = String::from(password);
    let space = pass.find(' ').unwrap();
    let mut rest = pass.split_off(space);
    let separator = pass.find('-').unwrap();
    let to: usize = pass.split_off(separator+1).parse().unwrap();
    pass.truncate(separator);
    let from: usize = pass.parse().unwrap();

    let mut pass_to_test = rest.split_off(3);
    let mut rest = rest.split_off(1);
    rest.truncate(1);
    let pass_to_test = pass_to_test.split_off(1);

    (from, to, rest, pass_to_test)
}

fn part1(passwords: &Vec<String>) {
    let mut result = 0;
    for password in passwords {
        let (from, to, rule, pass) = parse_password(&password);

        let mut number_of_chars = 0;
        for c in pass.chars() {
            if c.to_string() != rule {
                continue;
            }

            number_of_chars += 1;
        }
        if number_of_chars >= from && number_of_chars <= to {
            result += 1;
        }
    }

    println!("Part1: {}", result);
}

fn part2(passwords: &Vec<String>) {
    let mut result = 0;
    for password in passwords {
        let (from, to, rule, pass) = parse_password(&password);

        let first_char: char = pass.as_bytes()[from-1] as char;
        let second_char: char = pass.as_bytes()[to-1] as char;
        
        let first_correct = first_char.to_string() == rule;
        let second_correct = second_char.to_string() == rule;

        if  first_correct != second_correct  {
            result += 1;
        }
    }

    println!("Part2: {}", result);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input2.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split("\n");

    let mut passwords: Vec<String>  = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }
        passwords.push(line.to_string());
    }

    part1(&passwords);
    part2(&passwords);

    Ok(())
}
