use std::fs::File;
use std::io::{self, BufRead};

fn contains_all_fields(passport: &String) -> bool {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in &required_fields {
        if !passport.contains(field) {
            return false;
        }
    }

    return true;
}

fn part1(passports: &Vec<String>)
{
    let mut valid_passports = 0;
    for passport in passports {
        if contains_all_fields(passport) {
            valid_passports += 1;
        }
    }

    println!("Part1: {}", valid_passports);
}

fn check_byr(value: &str) -> bool {
    let real_value: i32 = value.parse().unwrap();
    return real_value >= 1920 && real_value <= 2002;
}

fn check_iyr(value: &str) -> bool {
    let real_value: i32 = value.parse().unwrap();
    return real_value >= 2010 && real_value <= 2020;
}

fn check_eyr(value: &str) -> bool {
    let real_value: i32 = value.parse().unwrap();
    return real_value >= 2020 && real_value <= 2030;
}

fn check_hgt(value: &str) -> bool {
    if !(value.ends_with("cm") || value.ends_with("in") || value.ends_with("cm ") || value.ends_with("in ")) {
        return false;
    }

    let (length, unit) = value.split_at(value.len()-2);
    let number: i32 = length.parse().unwrap();

    if unit == "cm" {
        return number >= 150 && number <= 193;
    }

    number >= 59 && number <= 76
}

fn check_hcl(value: &str) -> bool {
    if !value.starts_with("#") || value.len() != 7 {
        return false;
    }

    let color_value = value.split_at(1).1;
    let valid_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    for c in color_value.chars() {
        let mut valid = false;
        for valid_c in &valid_chars {
            if c == *valid_c {
                valid = true;
                break;
            }
        }

        if !valid {
            return false;
        }
    }

    return true;
}

fn check_ecl(value: &str) -> bool {
    let valid_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    for color in valid_colors {
        if color != value {
            continue;
        }

        return true;
    }

    return false;
}

fn check_pid(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    let valid_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    for c in value.chars() {
        let mut valid = false;
        for valid_c in &valid_chars {
            if c == *valid_c {
                valid = true;
                break;
            }
        }

        if !valid {
            return false;
        }
    }

    return true;
}

fn part2(passports: &Vec<String>)
{
    let mut valid_passwords = 0;
    for passport in passports {
        if !contains_all_fields(passport) {
            continue;
        }

        let mut valid = true;
        for field in passport.split(' ') {
            if field == "" {
                continue;
            }

            let (field_name, value) = field.split_at(field.find(':').unwrap());
            let clean_value = value.split_at(1).1;

            if field_name == "byr" {
                valid = check_byr(clean_value);
            } else if field_name == "iyr" {
                valid = check_iyr(clean_value);
            } else if field_name == "eyr" {
                valid = check_eyr(clean_value);
            } else if field_name == "hgt" {
                valid = check_hgt(clean_value);
            } else if field_name == "hcl" {
                valid = check_hcl(clean_value);
            } else if field_name == "ecl" {
                valid = check_ecl(clean_value);
            } else if field_name == "pid" {
                valid = check_pid(clean_value);
            } 

            if !valid {
                println!("{}: {}", field_name, clean_value);
                println!("{}", passport);
                break;
            }
        }

        if valid {
            valid_passwords += 1;
        }
    }

    println!("Part2: {}", valid_passwords);
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut passports: Vec<String>  = Vec::new();
    let mut passport: String = "".to_string();

    for line in lines {
        let line_str = line?;
        if line_str == "" {
            passports.push(passport);
            passport = "".to_string();
            continue;
        }

        passport += " ";
        passport += &line_str;
    }

    part1(&passports);
    part2(&passports);

    Ok(())
}
