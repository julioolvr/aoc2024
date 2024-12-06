use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let grid: Vec<Vec<char>> = read_lines()
        .expect("Unable to read lines")
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let word_search = WordSearch { grid };
    let part_1 = word_search.find_xmas();
    println!("Part 1: {}", part_1);

    let part_2 = word_search.find_x_mas();
    println!("Part 2: {}", part_2);
}

struct WordSearch {
    grid: Vec<Vec<char>>,
}

#[derive(PartialEq)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl WordSearch {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn find_xmas(&self) -> usize {
        let mut result = 0;

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.grid[y][x] == 'X' {
                    use Direction::*;
                    result += [N, S, W, E, NE, NW, SE, SW]
                        .iter()
                        .filter(|direction| self.find_rest(x, y, &['M', 'A', 'S'], direction))
                        .count()
                }
            }
        }

        result
    }

    fn find_x_mas(&self) -> usize {
        let mut result = 0;

        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.grid[y][x] == 'A' {
                    use Direction::*;

                    let first_mas = (self.find_rest(x, y, &['M'], &NW)
                        && self.find_rest(x, y, &['S'], &SE))
                        || (self.find_rest(x, y, &['M'], &SE) && self.find_rest(x, y, &['S'], &NW));
                    let second_mas = (self.find_rest(x, y, &['M'], &NE)
                        && self.find_rest(x, y, &['S'], &SW))
                        || (self.find_rest(x, y, &['M'], &SW) && self.find_rest(x, y, &['S'], &NE));

                    if first_mas && second_mas {
                        result += 1;
                    }
                }
            }
        }

        result
    }

    fn find_rest(&self, x: usize, y: usize, rest: &[char], direction: &Direction) -> bool {
        use Direction::*;

        if x == 0 {
            if *direction == W || *direction == SW || *direction == NW {
                return false;
            }
        }

        if x == self.width() - 1 {
            if *direction == E || *direction == NE || *direction == SE {
                return false;
            }
        }

        if y == 0 {
            if *direction == N || *direction == NE || *direction == NW {
                return false;
            }
        }

        if y == self.height() - 1 {
            if *direction == S || *direction == SE || *direction == SW {
                return false;
            }
        }

        let (next_x, next_y) = match direction {
            N => (x, y - 1),
            S => (x, y + 1),
            E => (x + 1, y),
            W => (x - 1, y),
            NW => (x - 1, y - 1),
            NE => (x + 1, y - 1),
            SW => (x - 1, y + 1),
            SE => (x + 1, y + 1),
        };

        if self.grid[next_y][next_x] != rest[0] {
            return false;
        }

        if rest.len() == 1 {
            return true;
        }

        self.find_rest(next_x, next_y, &rest[1..], direction)
    }
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
