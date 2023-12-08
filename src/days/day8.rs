use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayEight {}

impl AOCDay for DayEight {
    fn part_one(&self, input: &str) -> String {
        let mut rows = input.split('\n');
        let instructions: Vec<char> = rows.next().unwrap().chars().collect();
        let mut cycle = instructions.iter().cycle();
        rows.next();
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let reg = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").expect("failed to compile regex");
        for row in rows {
            let line = reg.captures(row).unwrap();
            map.insert(
                line.get(1).unwrap().as_str().to_string(),
                (
                    line.get(2).unwrap().as_str().to_string(),
                    line.get(3).unwrap().as_str().to_string()
                )
            );
        }
        let mut pos: String = "AAA".to_string();
        let mut step: u32 = 0;
        while pos != "ZZZ".to_string() {
            step += 1;
            let direction = cycle.next().unwrap();
            if direction == &'L' {
                pos = map.get(&pos).unwrap().0.clone();
            }
            else {
                pos = map.get(&pos).unwrap().1.clone();
            }
        }
        format!("{}", step)
    }

    fn part_two(&self, input: &str) -> String {
        let mut rows = input.split('\n');
        let instructions: Vec<char> = rows.next().unwrap().chars().collect();
        rows.next();
        let mut map: HashMap<String, (String, String)> = HashMap::new();
        let mut positions: Vec<String> = Vec::new();
        let reg = Regex::new(r"([1-9A-Z]{3}) = \(([1-9A-Z]{3}), ([1-9A-Z]{3})\)").expect("failed to compile regex");
        for row in rows {
            let line = reg.captures(row).unwrap();
            let pos: String = line.get(1).unwrap().as_str().to_string();
            map.insert(
                pos.clone(),
                (
                    line.get(2).unwrap().as_str().to_string(),
                    line.get(3).unwrap().as_str().to_string()
                )
            );
            if pos.chars().last().unwrap() == 'A' {
                positions.push(pos);
            }
        }
        let mut distances: Vec<u64> = Vec::new();
        for position in positions {
            let mut step: u64 = 0;
            let mut cycle = instructions.iter().cycle();
            let mut pos = position.clone();
            while pos.chars().last().unwrap() != 'Z' {
                step += 1;
                let direction = cycle.next().unwrap();
                if direction == &'L' {
                    pos = map.get(&pos).unwrap().0.clone();
                }
                else {
                    pos = map.get(&pos).unwrap().1.clone();
                }
            }
            distances.push(step);
        }
        let mut result: u64 = 1;
        for num in distances {
            result = lcm(result, num);
        }
        format!("{}", result)
    }
}
