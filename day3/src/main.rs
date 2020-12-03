use std::fs::File;
use std::io::prelude::*;

fn parse_input(input: &Vec<String>) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    for line in input {
        let mut row: Vec<bool> = Vec::new();
        for c in line.chars() {
            let readable: char = c as char;
            row.push(readable == '#');
        }
        result.push(row);
    }

    result
}

fn find_trees(trees: &Vec<Vec<bool>>, x_step: usize, y_step: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;

    let mut num_trees = 0;

    while y < trees.len() {
        if trees[y][x] {
            num_trees += 1;
        }

        x += x_step;
        if x >= trees[y].len() {
            x -= trees[y].len();
        }
        y += y_step;
    }

    num_trees
}

fn part1(input: &Vec<String>) {
    let trees = parse_input(input);
    println!("Part1: {}", find_trees(&trees, 3, 1));
}

fn part2(input: &Vec<String>) {
    let trees = parse_input(input);

    let mut mult: i64 = 1;

    mult *= find_trees(&trees, 1, 1);
    mult *= find_trees(&trees, 3, 1);
    mult *= find_trees(&trees, 5, 1);
    mult *= find_trees(&trees, 7, 1);
    mult *= find_trees(&trees, 1, 2);

    println!("Part2: {}", mult);
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.split("\n");

    let mut input: Vec<String>  = Vec::new();

    for line in lines {
        if line == "" {
            continue;
        }
        input.push(line.to_string());
    }

    part1(&input);
    part2(&input);

    Ok(())
}
