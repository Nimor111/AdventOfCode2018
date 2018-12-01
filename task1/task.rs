use std::fs::File;
use std::io::{BufRead, BufReader, Error};

use std::collections::HashSet;

fn read_file(filename: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(filename)?;

    let buffered = BufReader::new(f);

    Ok(buffered
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect())
}

fn accum(vec: Vec<i32>) -> Result<i32, Error> {
    Ok(vec.into_iter().fold(0, |accum, value| accum + value))
}

fn repeating_frequency(vec: Vec<i32>) -> i32 {
    let mut freq = 0;
    let mut frequencies = HashSet::new();

    loop {
        for num in &vec {
            if frequencies.contains(&freq) {
                return freq;
            }

            frequencies.insert(freq);
            freq += num;
        }
    }
}

pub fn main() -> Result<(), Error> {
    // println!("{:?}", read_file("data").and_then(accum));
    println!("{}", repeating_frequency(read_file("data")?));

    Ok(())
}
