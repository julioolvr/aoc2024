use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let lines = read_lines().expect("Unable to read lines");

    let mut first_location_ids = vec![];
    let mut second_location_ids = vec![];

    for line in lines {
        let location_ids: Vec<usize> = line
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        first_location_ids.push(location_ids[0]);
        second_location_ids.push(location_ids[1]);
    }

    first_location_ids.sort();
    second_location_ids.sort();

    let mut second_locations_map: HashMap<usize, usize> = HashMap::new();

    for location_id in &second_location_ids {
        *second_locations_map.entry(*location_id).or_insert(0) += 1;
    }

    let distances = first_location_ids
        .iter()
        .zip(second_location_ids.iter())
        .map(|(first, second)| usize::abs_diff(*first, *second));
    let part_1: usize = distances.sum();

    println!("Part 1 {}", part_1);

    let part_2: usize = first_location_ids
        .iter()
        .map(|location_id| second_locations_map.get(location_id).unwrap_or(&0) * location_id)
        .sum();
    println!("Part 2 {}", part_2);
}

fn read_lines() -> io::Result<io::Lines<io::BufReader<File>>> {
    let filename: String = env::args().skip(1).next().expect("Missing file path");
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
