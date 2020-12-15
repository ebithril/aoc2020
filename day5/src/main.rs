use std::fs::File;
use std::io::{self, BufRead};

fn get_seat_id(boarding_pass: &String) -> i32 {
    let mut low_row = 0;
    let mut high_row = 127;

    let mut low_column = 0;
    let mut high_column = 7;

    for c in boarding_pass.chars() {
        if c == 'B' {
            low_row = ((high_row + low_row) as f32 / 2.0_f32).ceil() as i32;
        } else if c == 'F' {
            high_row = ((high_row + low_row) as f32 / 2.0_f32).floor() as i32;
        } else if c == 'R' {
            low_column = ((high_column + low_column) as f32 / 2.0_f32).ceil() as i32;
        } else if c == 'L' {
            high_column = ((high_column + low_column) as f32 / 2.0_f32).floor() as i32;
        }
    }

    return high_row * 8 + high_column;
}

fn part1(boarding_passes: &Vec<String>)
{
    let mut highest_id = 0;
    for boarding_pass in boarding_passes {
        let pass_id = get_seat_id(&boarding_pass);
        if highest_id < pass_id {
            highest_id = pass_id;
        }
    }

    println!("Part1: {}", highest_id);
}

fn part2(boarding_passes: &Vec<String>)
{
    let mut pass_ids: Vec<i32> = Vec::new();
    for boarding_pass in boarding_passes {
        let pass_id = get_seat_id(&boarding_pass);
        pass_ids.push(pass_id);
    }

    pass_ids.sort();
    for i in 0..pass_ids.len()-1 {
        if pass_ids[i]+1 == pass_ids[i+1] {
            continue;
        }

        println!("Part2: {}", pass_ids[i]+1);
        break;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input5.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut boarding_passes: Vec<String>  = Vec::new();

    for line in lines {
        let line_str = line?;
        if line_str == "" {
            continue;
        }

        boarding_passes.push(line_str);
    }

    part1(&boarding_passes);
    part2(&boarding_passes);

    Ok(())
}
