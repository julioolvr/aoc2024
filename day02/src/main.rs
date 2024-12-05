use std::{
    env,
    fs::File,
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let lines = read_lines().expect("Unable to read lines");
    let reports: Vec<Report> = lines
        .map(|line| Report::from_str(&line.unwrap()).unwrap())
        .collect();
    let safe_reports = reports.iter().filter(|report| report.is_safe());
    println!("Part 1: {}", safe_reports.count());

    let safe_reports_with_dampener = reports
        .iter()
        .filter(|report| report.is_safe_with_dampener());
    println!("Part 2: {}", safe_reports_with_dampener.count());
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

    fn is_safe_with_dampener(&self) -> bool {
        self.is_safe() || (0..self.levels.len()).any(|level| self.dampening_level(level).is_safe())
    }

    fn dampening_level(&self, level: usize) -> Report {
        let mut new_levels = self.levels.clone();
        new_levels.remove(level);
        Report { levels: new_levels }
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
