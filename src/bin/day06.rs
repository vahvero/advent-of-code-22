/*
--- Day 6: Tuning Trouble ---
The preparations are finally complete; you and the Elves leave camp on foot and begin to make your way toward the star fruit grove.

As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says that it has many fancy features,
but the most important one to set up right now is the communication system.

However, because he's heard you have significant experience dealing with signal-based systems, he convinced the other Elves that
it would be okay to give you their one malfunctioning device - surely you'll have no problem fixing it.

As if inspired by comedic timing, the device emits a few colorful sparks.

To be able to communicate with the Elves, the device needs to lock on to their signal. The signal is a series of seemingly-random
characters that the device receives one at a time.

To fix the communication system, you need to add a subroutine to the device that detects a start-of-packet marker in the datastream.
In the protocol being used by the Elves, the start of a packet is indicated by a sequence of four characters that are all different.

The device will send your subroutine a datastream buffer (your puzzle input); your subroutine needs to identify the first position
where the four most recently received characters were all different. Specifically, it needs to report the number of characters
from the beginning of the buffer to the end of the first such four-character marker.

For example, suppose you receive the following datastream buffer:

mjqjpqmgbljsphdztnvjfqwrcgsmlb
After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker.
The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj.
Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm,
which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is
complete after 7 characters have been processed.

Here are a few more examples:

bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
How many characters need to be processed before the first start-of-packet marker is detected?
*/

use std::{collections::HashSet, fs};

const EXAMPLE0: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const EXAMPLE1: &str = "nppdvjthqldpwncqszvftbrmjlhg";
const EXAMPLE2: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const EXAMPLE3: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
const PROTOCOL_LENGTH: usize = 4;
const MSG_PROTOCOL_LENGTH: usize = 14;
const TRUE_INPUT: &str = "assets/day-06/true_input.txt";

fn main() {
    {
        let result = fetch_start_protocol(EXAMPLE0);
        assert!(result == 5, "{:?} != 5", result);
        let result = fetch_start_protocol(EXAMPLE1);
        assert!(result == 6, "{:?} != 6", result);
        let result = fetch_start_protocol(EXAMPLE2);
        assert!(result == 10, "{:?} != 10", result);
        let result = fetch_start_protocol(EXAMPLE3);
        assert!(fetch_start_protocol(EXAMPLE3) == 11, "{:?} != 11", result);
    }

    let in_file = fs::read_to_string(TRUE_INPUT).unwrap();
    let result = fetch_start_protocol(&in_file);
    println!("Start protocol index is {:?}", result);

    let result = start_message_protocol(&in_file);
    println!("Message protocol index is {:?}", result);
}

fn start_message_protocol(input: &str) -> usize {
    str_iter_chunk(input, MSG_PROTOCOL_LENGTH)
        .position(|chunk| {
            // It would be more efficient to collect as a cache list
            chunk.chars().into_iter().collect::<HashSet<_>>().len() == MSG_PROTOCOL_LENGTH
        })
        .unwrap()
        + MSG_PROTOCOL_LENGTH
}

fn fetch_start_protocol(input: &str) -> usize {
    str_iter_chunk(input, PROTOCOL_LENGTH)
        .position(|chunk| {
            let mut chars: Vec<char> = Vec::with_capacity(PROTOCOL_LENGTH);
            for character in chunk.chars() {
                chars.push(character);
            }
            chars[0] != chars[1]
                && chars[0] != chars[2]
                && chars[0] != chars[3]
                && chars[1] != chars[2]
                && chars[1] != chars[3]
                && chars[2] != chars[3]
        })
        .unwrap()
        + PROTOCOL_LENGTH
}

fn str_iter_chunk<'a>(
    input: &'a str,
    length: usize,
) -> std::iter::FlatMap<
    std::str::CharIndices<'a>,
    Option<&'a str>,
    impl FnMut((usize, char)) -> Option<&'a str>,
> {
    input.char_indices().flat_map(move |(from, _)| {
        input[from..]
            .char_indices()
            .skip(length - 1)
            .next()
            .map(|(to, c)| &input[from..from + to + c.len_utf8()])
    })
}
