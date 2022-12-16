use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() {
        let example_file = "assets/example_file.txt";
        if let Ok(lines) = read_lines(example_file) {
            for line in lines {
                if let Ok(_ip) = line {}
            }
        }
    }
}

// Source: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

