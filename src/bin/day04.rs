use advent_of_code_22::read_lines;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;

const DUMMY_FILE: &str = "assets/day-04/dummy_input.txt";
const TRUE_FILE: &str = "assets/day-04/true_input.txt";

fn main() {
    assert_eq!(contain1(2, 4, 6, 8), false);
    assert_eq!(contain1(2, 3, 4, 5), false);
    assert_eq!(contain1(5, 7, 7, 9), false);
    assert_eq!(contain1(2, 8, 3, 7), true);
    assert_eq!(contain1(6, 6, 4, 6), true);
    assert_eq!(contain1(2, 6, 4, 8), false);

    let dummy = read_lines(DUMMY_FILE).unwrap();
    let dummy_resp_1 = process_task(dummy, &contain1);

    assert!(
        dummy_resp_1 == 2,
        "`dummy_resp_1` should be {} but is {}",
        2,
        dummy_resp_1
    );

    let true_lines = read_lines(TRUE_FILE).unwrap();
    let true_resp1 = process_task(true_lines, &contain1);
    println!("True input output {}", true_resp1);

    assert_eq!(contain2(2, 4, 6, 8), false);
    assert_eq!(contain2(2, 3, 4, 5), false);
    assert_eq!(contain2(5, 7, 7, 9), true);
    assert_eq!(contain2(2, 8, 3, 7), true);
    assert_eq!(contain2(6, 6, 4, 6), true);
    assert_eq!(contain2(2, 6, 4, 8), true);

    let true_lines = read_lines(TRUE_FILE).unwrap();
    let true_resp1 = process_task(true_lines, &contain2);
    println!("True input output {}", true_resp1);
}

fn contain1(start0: i32, end0: i32, start1: i32, end1: i32) -> bool {
    (start0 >= start1 && end0 <= end1) || (start1 >= start0 && end1 <= end0)
}

fn contain2(start0: i32, end0: i32, start1: i32, end1: i32) -> bool {
    end0 >= start1 && start0 <= end1
}

fn process_task(
    lines: std::io::Lines<BufReader<File>>,
    contain: &dyn Fn(i32, i32, i32, i32) -> bool,
) -> u64 {
    let mut count: u64 = 0;
    let splitter_re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    for line in lines {
        match line {
            Ok(line) => {
                let cap = splitter_re.captures(&line);
                match cap {
                    Some(cap) => {
                        if let (Some(start0), Some(start1), Some(end0), Some(end1)) =
                            (cap.get(1), cap.get(3), cap.get(2), cap.get(4))
                        {
                            let start0 = start0.as_str().parse::<i32>().unwrap();
                            let start1 = start1.as_str().parse::<i32>().unwrap();
                            let end0 = end0.as_str().parse::<i32>().unwrap();
                            let end1 = end1.as_str().parse::<i32>().unwrap();

                            let valid = contain(start0, end0, start1, end1);
                            if valid {
                                count += 1;
                            }
                        } else {
                            panic!("REGEX did not capture all groups")
                        }
                    }
                    _ => {
                        panic!("Regex did not match line {}", line);
                    }
                }
            }
            _ => panic!("Error reading file"),
        }
    }
    count
}
