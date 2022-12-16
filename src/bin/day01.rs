use advent_of_code_22::read_lines;
use std::fs::File;
use std::io::{self};

const DUMMY_INPUT: &str = "assets/day-01/example_input.txt";
const TRUE_INPUT: &str = "assets/day-01/true_input.txt";

fn main() {
    let test_answer = (24_000, 11_000, 10_000);

    let lines = read_lines(DUMMY_INPUT);
    match lines {
        Ok(lines) => {
            let result = most_calories(lines);
            assert!(
                result == (24_000, 11_000, 10_000),
                "Answer {:?} != {:?}",
                result,
                test_answer
            );
            println!("{:?}", result);
        }
        Err(lines) => {
            println!("{:?}", lines);
        }
    }

    let lines = read_lines(TRUE_INPUT);
    match lines {
        Ok(lines) => {
            let result = most_calories(lines);
            println!("{:?}", result);
            println!("{:?}", result.0 + result.1 + result.2);
        }
        Err(lines) => {
            println!("{:?}", lines);
        }
    }
}

fn most_calories(lines: io::Lines<io::BufReader<File>>) -> (i32, i32, i32) {
    let mut best_three = (0, 0, 0);
    let mut current_sum = 0;

    for line in lines {
        if let Ok(ip) = line {
            if !ip.is_empty() {
                current_sum += ip.parse::<i32>().unwrap();
            } else {
                insert_best(current_sum, &mut best_three);
                current_sum = 0;
            }
        }
    }

    insert_best(current_sum, &mut best_three);

    best_three
}

fn insert_best(current_sum: i32, best_three: &mut (i32, i32, i32)) {
    if current_sum > best_three.0 {
        best_three.2 = best_three.1;
        best_three.1 = best_three.0;
        best_three.0 = current_sum;
    } else if current_sum > best_three.1 {
        best_three.2 = best_three.1;
        best_three.1 = current_sum;
    } else if current_sum > best_three.2 {
        best_three.2 = current_sum;
    }
}
