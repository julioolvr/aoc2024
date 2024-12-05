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

    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut part_2 = 0;
    let mut is_disabled = false;

    for capture in re.captures_iter(&memory) {
        match &capture[0] {
            "do()" => is_disabled = false,
            "don't()" => is_disabled = true,
            _ => {
                if !is_disabled {
                    part_2 +=
                        capture[1].parse::<usize>().unwrap() * capture[2].parse::<usize>().unwrap();
                }
            }
        }
    }
    println!("Part 2: {}", part_2);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
