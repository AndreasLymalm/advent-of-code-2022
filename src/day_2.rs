use crate::common::Solution;
use std::io::Error;

pub struct SolutionDay2 {
    pub lines: Vec<String>,
}

impl Solution for SolutionDay2 {
    fn part_1(&self) -> Result<String, Error>  {
        let mut result = 0;
        for i in 0..self.lines.len() {
            let line: Vec<&str> = self.lines[i].split(" ").collect();
            let opponent = line[0];
            let me = line[1];

            if me == "X" { 
                result += 1;
                if opponent == "A" { result += 3 }
                if opponent == "C" { result += 6 }
            }
            if me == "Y" { 
                result += 2;
                if opponent == "B" { result += 3 }
                if opponent == "A" { result += 6 } 
            }
            if me == "Z" { 
                result += 3;
                if opponent == "C" { result += 3 }
                if opponent == "B" { result += 6 } 
            }
        }
        
        return Ok(result.to_string());
    }
    
    fn part_2(&self) -> Result<String, Error>  {
        let mut result = 0;
        for i in 0..self.lines.len() {
            let line: Vec<&str> = self.lines[i].split(" ").collect();
            let opponent = line[0];
            let strategy = line[1];
            let mut me: &str = "";
            if strategy == "X" { // Lose
                if opponent == "A" { me = "Z"; }
                if opponent == "B" { me = "X"; }
                if opponent == "C" { me = "Y"; }
            }
            else if strategy == "Y" { // Draw
                if opponent == "A" { me = "X"; }
                if opponent == "B" { me = "Y"; }
                if opponent == "C" { me = "Z"; }
            }
            else if strategy == "Z" { // Win
                if opponent == "A" { me = "Y"; }
                if opponent == "B" { me = "Z"; }
                if opponent == "C" { me = "X"; }
            }

            if me == "X" { 
                result += 1;
                if opponent == "A" { result += 3 }
                if opponent == "C" { result += 6 }
            }
            if me == "Y" { 
                result += 2;
                if opponent == "B" { result += 3 }
                if opponent == "A" { result += 6 } 
            }
            if me == "Z" { 
                result += 3;
                if opponent == "C" { result += 3 }
                if opponent == "B" { result += 6 } 
            }
        }
        
        return Ok(result.to_string());
    }
}