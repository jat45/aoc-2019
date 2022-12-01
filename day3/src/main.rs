use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
enum Movement {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64),
}

#[derive(Debug, PartialEq)]
struct Path {
    movements: Vec<Movement>,
}

fn read_input(filepath: &str) -> Vec<String> {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .collect()
}

fn parse(input: Vec<String>) -> Vec<Path> {
    let mut paths = Vec::new();
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let input = vec![];
        let expected = vec![];
        let parsed = parse(input);
        assert_eq!(parsed, expected)
    }
}
