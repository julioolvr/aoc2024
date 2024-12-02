use std::{
    env,
    fs::File,
    io::{self, BufRead}, str::FromStr,
};

fn main() {
    let lines = read_lines().expect("Unable to read lines");
    let reports = lines.map(|line| Report::from_str(&line.unwrap()).unwrap());
    let safe_reports = reports.filter(|report| report.is_safe());
    println!("Part 1: {}", safe_reports.count());
}

struct Report {
    levels: Vec<usize>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let is_decreasing = self.levels[0] > self.levels[1];

        self.levels.windows(2).all(|window| {
            let a = window[0];
            let b = window[1];
            a != b && (a > b) == is_decreasing && usize::abs_diff(a, b) <= 3
        })
    }
}

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .split_whitespace()
            .map(|level| level.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Report { levels })
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
