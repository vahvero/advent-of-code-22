use advent_of_code_22::read_lines;
use std::fs::File;
use std::io::{self};

const DUMMY_INPUT: &str = "assets/day-02/dummy_input.txt";
const TRUE_INPUT: &str = "assets/day-02/true_input.txt";

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;
const D: i32 = 3;
const W: i32 = 6;
const L: i32 = 0;

fn match_line_task2(line: String) -> i32 {
    match &line[..] {
        "A X" => L + SCISSORS, // ROCK
        "A Y" => D + ROCK,
        "A Z" => W + PAPER,
        "B X" => L + ROCK, // PAPER
        "B Y" => D + PAPER,
        "B Z" => W + SCISSORS,
        "C X" => L + PAPER, // SCISSORS
        "C Y" => D + SCISSORS,
        "C Z" => W + ROCK,
        _ => panic!("Invalid input"),
    }
}

fn match_line_task1(line: String) -> i32 {
    match &line[..] {
        "A X" => D + ROCK,
        "A Y" => W + PAPER,
        "A Z" => L + SCISSORS,
        "B X" => L + ROCK,
        "B Y" => D + PAPER,
        "B Z" => W + SCISSORS,
        "C X" => W + ROCK,
        "C Y" => L + PAPER,
        "C Z" => D + SCISSORS,
        _ => panic!("Invalid input"),
    }
}

fn calculate_strategy(
    lines: io::Lines<io::BufReader<File>>,
    match_line: &dyn Fn(String) -> i32,
) -> i32 {
    let mut my_score = 0;
    for line in lines {
        match line {
            Ok(line) => {
                my_score += match_line(line);
            }
            Err(line) => {
                panic!("{:?}", line);
            }
        }
    }
    my_score
}

fn main() {
    let lines = read_lines(DUMMY_INPUT);

    match lines {
        Ok(lines) => {
            let dummy_result = calculate_strategy(lines, &match_line_task1);
            assert!(dummy_result == 15, "{} != 15", dummy_result);
        }
        Err(lines) => {
            println!("{:?}", lines);
        }
    }

    println!(
        "{}",
        calculate_strategy(read_lines(TRUE_INPUT).unwrap(), &match_line_task1)
    );

    let lines = read_lines(DUMMY_INPUT);

    match lines {
        Ok(lines) => {
            let dummy_result = calculate_strategy(lines, &match_line_task2);
            assert!(dummy_result == 12, "{} != 12", dummy_result);
        }
        Err(lines) => {
            println!("{:?}", lines);
        }
    }

    println!(
        "{}",
        calculate_strategy(read_lines(TRUE_INPUT).unwrap(), &match_line_task2)
    );
}
