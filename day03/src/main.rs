use regex::Regex;
use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let memory = read_lines()
        .expect("Unable to read lines")
        .map(|line| line.unwrap())
        .next()
        .unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let part_1: usize = re
        .captures_iter(&memory)
        .map(|capture| capture.extract())
        .map(|(_, [a, b])| a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap())
        .sum();
    println!("Part 1: {}", part_1);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
