use maplit::hashmap;
use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayEighteen {}


impl AOCDay for DayEighteen {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let directions: HashMap<char, (isize, isize)> = hashmap!{
            'U' => (-1, 0),
            'D' => (1, 0),
            'R' => (0, 1),
            'L' => (0, -1),
        };
        let mut area: isize = 0;
        let mut current_pos: (isize, isize) = (0, 0);
        let mut step: isize = 0;
        for row in rows {
            let mut row_splitted = row.split_whitespace();
            let direction: char = row_splitted.next().unwrap().chars().next().unwrap();
            let length = row_splitted.next().unwrap().parse::<usize>().unwrap();
            for _ in 1..length+1 {
                step += 1;
                current_pos = (
                    (current_pos.0 + directions[&direction].0),
                    (current_pos.1 + directions[&direction].1),
                );
                area += current_pos.0 * directions[&direction].1;
            }
        }
        format!("{}", (area - step / 2).abs() + 1)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let directions: HashMap<char, (isize, isize)> = hashmap!{
            '3' => (-1, 0),
            '1' => (1, 0),
            '0' => (0, 1),
            '2' => (0, -1),
        };
        let mut area: isize = 0;
        let mut current_pos: (isize, isize) = (0, 0);
        let mut step: isize = 0;
        for row in rows {
            let mut row_splitted = row.split_whitespace();
            row_splitted.next();
            row_splitted.next();
            let hexadecimal: Vec<char> = row_splitted.next().unwrap().chars().collect();
            let direction: char = hexadecimal[7];
            let length_string: String = hexadecimal[2..7].iter().collect();
            let length: isize = i64::from_str_radix(&length_string, 16).unwrap() as isize;
            step += length;
            current_pos = (
                (current_pos.0 + length * directions[&direction].0),
                (current_pos.1 + length * directions[&direction].1),
            );
            area += current_pos.0 * directions[&direction].1 * length;
        }
        format!("{}", (area - step / 2).abs() + 1)
    }
}