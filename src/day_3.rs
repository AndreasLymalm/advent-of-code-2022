use crate::common::Solution;
use std::{io::Error, collections::HashMap, collections::HashSet};

pub struct SolutionDay3 {
    pub lines: Vec<String>,
}

impl Solution for SolutionDay3 {
    fn part_1(&self) -> Result<i32, Error>  {
        let mut result: i32 = 0;
        for i in 0..self.lines.len() {
            let line: &String = self.lines.get(i).unwrap();
            let second_compartment_start = line.len() / 2;

            let mut misplaced_item = char::default();
            for first_i in 0..second_compartment_start {
                let first_char = line.chars().nth(first_i).unwrap();
                for second_i in second_compartment_start..line.len() {
                    let second_char = line.chars().nth(second_i).unwrap();
                    if first_char == second_char {
                        misplaced_item = first_char;
                        break;
                    }
                }
                if misplaced_item != char::default() {
                    break;
                }
            }
            // Disregards lower/uppercase
            let priority = (misplaced_item.to_digit(36).unwrap() - 9) as i32;
            // Adjust uppercase letters
            result += priority + (if misplaced_item.is_lowercase() { 0 } else { 26 });
        }
        return Ok(result);
    }
    
    fn part_2(&self) -> Result<i32, Error>  {
        let mut result = 0;
        let mut group_item_types: HashMap<char, i32> = HashMap::new();
        for i in 0..self.lines.len() {
            let line: &String = self.lines.get(i).unwrap();

            // Clear item type count for new groups
            if i % 3 == 0 {
                group_item_types.clear();
            }

            // Count unique item types for group member
            let mut unique_item_types: HashSet<char> = HashSet::new();
            for k in 0..line.len() {
                let item_type = line.chars().nth(k).unwrap();
                unique_item_types.insert(item_type);
            }
            for item_type in unique_item_types {
                group_item_types.entry(item_type)
                    .and_modify(|item_type| *item_type += 1)
                    .or_insert(1);
            }

            // Determine badge group if last group member
            if i % 3 == 2 {
                let type_of_badge = group_item_types.iter()
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .map(|(k, _v)| k)
                    .unwrap();

                // Disregards lower/uppercase
                let priority = (type_of_badge.to_digit(36).unwrap() - 9) as i32;
                // Adjust uppercase letters
                result += priority + (if type_of_badge.is_lowercase() { 0 } else { 26 });
            }
        }
        return Ok(result);
    }
}