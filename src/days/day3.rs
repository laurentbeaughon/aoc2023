use regex::Regex;
use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayThree {}

fn process_input(input: &str) -> Vec<(i32, i32)> {
    let rows = input.split('\n');
    let char_list = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut symbols: Vec<(i32, i32)> = Vec::new();
    let mut i: i32 = 0;
    for row in rows {
        let mut j: i32 = 0;
        let char_vec: Vec<char> = row.chars().collect();
        for c in char_vec {
            match char_list.iter().position(|&m| m == c) {
                Some(_) => {}
                None => {symbols.push((i, j))}
            }
            j += 1;
        }
        i += 1
    }
    symbols
}

impl AOCDay for DayThree {
    fn part_one(&self, input: &str) -> String {
        let symbols = process_input(input);
        let rows = input.split('\n');
        let regex = Regex::new(r"(\d+)").expect("failed to compile regex");
        let mut valid_numbers: Vec<i32> = Vec::new();
        let mut i: i32 = 0;
        for row in rows {
            for capture in regex.captures_iter(row) {
                let number = capture.get(1).expect("Invalid capture group").as_str();
                let start_position: i32 = capture.get(0).expect("Invalid capture").start() as i32;
                let end_position: i32 = capture.get(0).expect("Invalid capture").end() as i32;
                let number_as_int: i32 = number.parse().unwrap();
                let mut is_valid: bool = false;
                for pos in start_position..end_position {
                    for x in [-1, 0, 1] {
                        for y in [-1, 0, 1] {
                            if symbols.contains(&(i + x, pos + y)) {
                                is_valid = true;
                            }
                        }
                    }
                }
                if is_valid {
                    valid_numbers.push(number_as_int);
                }
            }
            i += 1;
        }
        format!("{}", valid_numbers.iter().sum::<i32>())
    }

    fn part_two(&self, input: &str) -> String {
        let symbols = process_input(input);
        let mut hm = HashMap::new();
        for symbol in &symbols {
            hm.insert(symbol, Vec::new());
        }
        let rows = input.split('\n');
        let regex = Regex::new(r"(\d+)").expect("failed to compile regex");
        let mut i: i32 = 0;
        for row in rows {
            for capture in regex.captures_iter(row) {
                let number = capture.get(1).expect("Invalid capture group").as_str();
                let start_position: i32 = capture.get(0).expect("Invalid capture").start() as i32;
                let end_position: i32 = capture.get(0).expect("Invalid capture").end() as i32;
                let number_as_int: i32 = number.parse().unwrap();
                let mut is_valid: bool = false;
                let mut match_symbol: (i32, i32) = (0, 0);
                for pos in start_position..end_position {
                    for x in [-1, 0, 1] {
                        for y in [-1, 0, 1] {
                            if symbols.contains(&(i + x, pos + y)) {
                                is_valid = true;
                                match_symbol = (i + x, pos + y);
                            }
                        }
                    }
                }
                if is_valid {
                    if let Some(vector) = hm.get_mut(&match_symbol) {
                        vector.push(number_as_int);
                    };
                }
            }
            i += 1;
        }
        let result: i32 = hm
            .iter()
            .filter(|(_, v)| v.len() == 2)
            .map(|(_, v)| v.iter().product::<i32>())
            .sum();
        format!("{}", result)
    }
}
