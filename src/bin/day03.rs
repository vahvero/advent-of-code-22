use advent_of_code_22::read_lines;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DUMMY_FILE: &str = "assets/day-03/dummy_input.txt";
const TRUE_FILE: &str = "assets/day-03/true_input.txt";

fn main() {
    let charmap: HashMap<char, i32> = ALPHABET.chars().into_iter()
        .enumerate()
        .map(|x| (x.1, (x.0 + 1) as i32))
        .collect();

    assert!(charmap[&'p'] == 16, "Should be {}, is {}",16,  charmap[&'p']);
    assert!(charmap[&'L'] == 38, "Should be {}, is {}",38,  charmap[&'L']);
    assert!(charmap[&'P'] == 42, "Should be {}, is {}",42,  charmap[&'P']);
    assert!(charmap[&'v'] == 22, "Should be {}, is {}",22,  charmap[&'v']);
    assert!(charmap[&'t'] == 20, "Should be {}, is {}",20,  charmap[&'t']);
    assert!(charmap[&'s'] == 19, "Should be {}, is {}",19,  charmap[&'s']);

    let lines = read_lines(DUMMY_FILE).unwrap();
    println!("charmap: {:?}", charmap);
    let get_priority = |letter: &char| {
        charmap[letter]
    };

    let value = process_task1(lines, &get_priority);
    assert!(value == 157);

    let lines = read_lines("assets/day-03/true_input.txt").unwrap();
    let value = process_task1(lines, &get_priority);
    println!("Task1 = {}", value);

    let lines = read_lines(DUMMY_FILE).unwrap();
    let value = process_task2(lines, &get_priority);
    assert!(value == 70, "{} != 70", value);

    let lines = read_lines(TRUE_FILE).unwrap();
    let value = process_task2(lines, &get_priority);
    println!("Task2 = {}", value);

}



fn process_task2(mut lines: std::io::Lines<BufReader<File>>, get_priority: &dyn Fn(&char) -> i32) -> i32 {
    let mut total_priority = 0;

    while let (Some(line0), Some(line1), Some(line2)) = (lines.next(), lines.next(), lines.next()) {
        // Split from the middle
        let first: HashSet<char> = line0.unwrap().chars().collect();
        let second: HashSet<char> = line1.unwrap().chars().collect();
        let third: HashSet<char> = line2.unwrap().chars().collect();

        for letter in &(&first & &second) & &third {
            total_priority += get_priority(&letter);
        }
    }
    total_priority
}


fn process_task1(lines: std::io::Lines<BufReader<File>>, get_priority: &dyn Fn(&char) -> i32) -> i32 {
    let mut total_priority = 0;
    for line in lines {
        let line = line.unwrap();
        // Split from the middle
        let first: HashSet<char> = line[0..line.len() / 2].chars().collect();
        let second: HashSet<char> = line[line.len() / 2..].chars().collect();

        for letter in first.intersection(&second) {
            total_priority += get_priority(letter);
        }
    }
    total_priority
}
