use std::fs::File;
use std::io::{prelude::*, BufReader, Error};

pub trait Solution {
    fn part_1(&self) -> Result<String, Error>;
    fn part_2(&self) -> Result<String, Error>;
}

pub fn read_input_for_day(filename: String) -> Vec<String> {
    let file = File::open(filename)
        .expect("File does not exist");

    BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
