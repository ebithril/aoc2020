use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn part1(answers: &Vec<String>)
{
    let mut num_yeses = 0;
    for answer in answers {
        let mut checked_answers = HashSet::new();
        for c in answer.chars() {
            if c == '\n' {
                continue;
            }

            if checked_answers.contains(&c) {
                continue;
            }

            checked_answers.insert(c);
        }

        num_yeses += checked_answers.len();
    }

    println!("Part1: {}", num_yeses);
}

fn part2(answers: &Vec<String>)
{
    for answer in answers {
        let group_answers = answer.split("\n");
        for thing in group_answers {
            println!("{}", thing);
        }
        println!("Hello, WOrld!");
        break;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input6.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut answers: Vec<String>  = Vec::new();
    let mut answer: String = "".to_string();

    for line in lines {
        let line_str = line?;
        if line_str == "" {
            answers.push(answer);
            answer = "".to_string();
            continue;
        }

        answer += &line_str;
        answer += "\n";
    }

    part1(&answers);
    part2(&answers);

    Ok(())
}
