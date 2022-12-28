use crate::common::Solution;
use std::{io::Error};

pub struct SolutionDay4 {
    pub lines: Vec<String>,
}

impl Solution for SolutionDay4 {
    fn part_1(&self) -> Result<String, Error>  {
        let mut result: i32 = 0;
        for i in 0..self.lines.len() {
            // Parse line
            let elf_pair: Vec<&str> = self.lines[i].split(",").collect();
            let elf_1: Vec<&str> = elf_pair[0].split("-").collect();
            let elf_2: Vec<&str> = elf_pair[1].split("-").collect();
            let elf_1_min = elf_1[0].parse::<i32>().unwrap();
            let elf_1_max = elf_1[1].parse::<i32>().unwrap();
            let elf_2_min = elf_2[0].parse::<i32>().unwrap();
            let elf_2_max = elf_2[1].parse::<i32>().unwrap();

            if (elf_1_min <= elf_2_min && elf_2_max <= elf_1_max) 
              || (elf_2_min <= elf_1_min && elf_1_max <= elf_2_max) {
                result += 1;
            }
        }
        return Ok(result.to_string());
    }
    
    fn part_2(&self) -> Result<String, Error>  {
        let mut result: i32 = 0;
        for i in 0..self.lines.len() {
            // Parse line
            let elf_pair: Vec<&str> = self.lines[i].split(",").collect();
            let elf_1: Vec<&str> = elf_pair[0].split("-").collect();
            let elf_2: Vec<&str> = elf_pair[1].split("-").collect();
            let elf_1_min = elf_1[0].parse::<i32>().unwrap();
            let elf_1_max = elf_1[1].parse::<i32>().unwrap();
            let elf_2_min = elf_2[0].parse::<i32>().unwrap();
            let elf_2_max = elf_2[1].parse::<i32>().unwrap();

            if (elf_2_min <= elf_1_min && elf_1_min <= elf_2_max) 
              || (elf_2_min <= elf_1_max && elf_1_max <= elf_2_max)
              || (elf_1_min <= elf_2_min && elf_2_min <= elf_1_max)
              || (elf_1_min <= elf_2_max && elf_2_max <= elf_1_max) {
                result += 1;
            }
        }
        return Ok(result.to_string());
    }
}