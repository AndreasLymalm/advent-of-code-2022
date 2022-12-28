use crate::common::Solution;
use std::io::Error;

pub struct SolutionDay1 {
    pub lines: Vec<String>,
}

impl Solution for SolutionDay1 {
    fn part_1(&self) -> Result<String, Error>  {
        let mut max_calories = 0;
        let mut current_elfs_calories = 0;
        for i in 0..self.lines.len() {
            if self.lines[i].len() == 0 {
                if current_elfs_calories > max_calories {
                    max_calories = current_elfs_calories;
                }
                current_elfs_calories = 0;
            }
            else {
                let food_item_calories = self.lines[i].parse::<i32>().unwrap();
                current_elfs_calories += food_item_calories;
            }
        }
        
        return Ok(max_calories.to_string());
    }
    
    fn part_2(&self) -> Result<String, Error>  {
        let mut top_1_calories = 0;
        let mut top_2_calories = 0;
        let mut top_3_calories = 0;
        let mut current_elfs_calories = 0;
        for i in 0..self.lines.len() {
            if self.lines[i].len() == 0 {
                if current_elfs_calories > top_3_calories {
                    top_3_calories = current_elfs_calories;
                }
                if current_elfs_calories > top_2_calories {
                    top_3_calories = top_2_calories;
                    top_2_calories = current_elfs_calories;
                }
                if current_elfs_calories > top_1_calories {
                    top_2_calories = top_1_calories;
                    top_1_calories = current_elfs_calories;
                }
                current_elfs_calories = 0;
            }
            else {
                let food_item_calories = self.lines[i].parse::<i32>().unwrap();
                current_elfs_calories += food_item_calories;
            }
        }
        
        let result = top_1_calories + top_2_calories + top_3_calories;
        return Ok(result.to_string());
        
        // return Err(Error::new(ErrorKind::Other, "Solution not found"));
    }
}