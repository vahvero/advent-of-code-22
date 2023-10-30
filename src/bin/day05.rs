use advent_of_code_22::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self};

const TRUE_INPUT: &str = "assets/day-05/true_input.txt";
const DUMMY_INPUT: &str = "assets/day-05/dummy_input.txt";

fn main() {
    // First part
    {
        let mut lines = read_lines(DUMMY_INPUT).unwrap();
        let result = extract_top_crates(&mut lines, &execute_move_crate_mover9000);
        println!("End result {:?}", result);
        assert!(get_message(result) == "CMZ");

        let mut lines = read_lines(TRUE_INPUT).unwrap();
        let result = extract_top_crates(&mut lines, &execute_move_crate_mover9000);
        println!("End result {:?}", result);
        println!("Message: {}", get_message(result));
    }
    {
        let mut lines = read_lines(DUMMY_INPUT).unwrap();
        let result = extract_top_crates(&mut lines, &execute_move_crate_mover9001);
        println!("End result {:?}", result);
        assert!(get_message(result) == "MCD");
        let mut lines = read_lines(TRUE_INPUT).unwrap();
        let result = extract_top_crates(&mut lines, &execute_move_crate_mover9001);
        println!("End result {:?}", result);
        println!("Message: {}", get_message(result));
    }
}

fn get_message(result: HashMap<usize, VecDeque<char>>) -> String {
    // Getting message
    let mut message = String::with_capacity(12);
    let mut keys: Vec<&usize> = Vec::from_iter(result.keys());
    keys.sort();
    for i in keys {
        let queue = result.get(&i).unwrap();
        message.push(*queue.back().unwrap());
    }
    message
}

fn is_empty(slice: &[char]) -> bool {
    !slice.iter().any(|c| !c.is_ascii_whitespace())
}

fn get_starting_position(
    lines: &mut io::Lines<io::BufReader<File>>,
) -> HashMap<usize, VecDeque<char>> {
    let mut crates = HashMap::new();
    for line in lines {
        // Length should be 36
        match line {
            Ok(line) => {
                if line.is_empty() || line.chars().any(|x| x.is_numeric()) {
                    break;
                }
                for (idx, _crate) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                    if !is_empty(_crate) {
                        crates
                            .entry(idx + 1)
                            .or_insert_with(|| VecDeque::new())
                            .push_front(_crate[1]);
                    }
                }
            }
            Err(line) => {
                println!("Error on line {:?}", line);
            }
        }
    }

    crates
}

fn execute_move_crate_mover9000(
    crate_positions: &mut HashMap<usize, VecDeque<char>>,
    amount: usize,
    from: usize,
    to: usize,
) {
    let mut source_boxes: Vec<char> = Vec::with_capacity(amount);
    {
        let source = crate_positions.get_mut(&from).unwrap();
        let _source_boxes = source.drain(source.len() - amount..);
        source_boxes.extend(_source_boxes);
    }
    let target = crate_positions.get_mut(&to).unwrap();
    target.extend(source_boxes.iter().rev());
}
fn execute_move_crate_mover9001(
    crate_positions: &mut HashMap<usize, VecDeque<char>>,
    amount: usize,
    from: usize,
    to: usize,
) {
    let mut source_boxes: Vec<char> = Vec::with_capacity(amount);
    {
        let source = crate_positions.get_mut(&from).unwrap();
        let _source_boxes = source.drain(source.len() - amount..);
        source_boxes.extend(_source_boxes);
    }
    let target = crate_positions.get_mut(&to).unwrap();
    target.extend(source_boxes);
}

fn extract_top_crates(
    lines: &mut io::Lines<io::BufReader<File>>,
    move_function: &dyn Fn(&mut HashMap<usize, VecDeque<char>>, usize, usize, usize),
) -> HashMap<usize, VecDeque<char>> {
    let re = Regex::new(r"move (?<move>\d+) from (?<from>\d+) to (?<to>\d+)").unwrap();

    let mut crate_positions = get_starting_position(lines);
    println!("Starting pos {:?}", crate_positions);

    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }
                let result = re.captures(&line);
                match result {
                    Some(result) => {
                        let _move = result["move"].parse::<usize>().unwrap();
                        let _from = result["from"].parse::<usize>().unwrap();
                        let _to = result["to"].parse::<usize>().unwrap();
                        move_function(&mut crate_positions, _move, _from, _to);
                    }
                    _ => {}
                }
            }
            _ => break,
        }
    }
    crate_positions
}
