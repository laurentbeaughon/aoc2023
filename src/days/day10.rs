use maplit::hashmap;
use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayTen {}

impl AOCDay for DayTen {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut field: Vec<Vec<char>> = Vec::new();
        let mut start_pos: (usize, usize) = (0, 0);
        for (i, row) in rows.enumerate() {
            let line: Vec<char> = row.chars().collect();
            if line.contains(&'S') {
                start_pos = (i, line.iter().position(|&x| x == 'S').unwrap())
            }
            field.push(line);
        }
        let new_directions = hashmap! {
            'N' => hashmap! {
                '7' => 'W',
                '|' => 'N',
                'F' => 'E',
            },
            'E' => hashmap! {
                '7' => 'S',
                '-' => 'E',
                'J' => 'N',
            },
            'S' => hashmap! {
                'L' => 'E',
                '|' => 'S',
                'J' => 'W',
            },
            'W' => hashmap! {
                'L' => 'N',
                '-' => 'W',
                'F' => 'S',
            }
        };
        let mut directions: HashMap<char, (isize, isize)> = HashMap::new();
        directions.insert('N', (-1, 0));
        directions.insert('S', (1, 0));
        directions.insert('E', (0, 1));
        directions.insert('W', (0, -1));
        let mut current_pos: (usize, usize) = (start_pos.0, start_pos.1);
        let mut step: usize = 0;
        let mut direction: char = 'N';
        if field[start_pos.0][start_pos.1 + 1] == '7' || field[start_pos.0][start_pos.1 + 1] == '-' || field[start_pos.0][start_pos.1 + 1] == 'J' {
            direction = 'E';
        }
        if field[start_pos.0][start_pos.1 - 1] == 'L' || field[start_pos.0][start_pos.1 - 1] == '-' || field[start_pos.0][start_pos.1 - 1] == 'F' {
            direction = 'W';
        }
        while current_pos != start_pos || step == 0 {
            step += 1;
            current_pos = (
                (current_pos.0 as isize + directions[&direction].0) as usize,
                (current_pos.1 as isize + directions[&direction].1) as usize,
            );
            direction = *new_directions
                .get(&direction).unwrap()
                .get(&field[current_pos.0][current_pos.1]).unwrap_or(&'S')
        }
        format!("{}", step / 2)
    }

    fn part_two(&self, input: &str) -> String {
                let rows = input.split('\n');
        let mut field: Vec<Vec<char>> = Vec::new();
        let mut start_pos: (usize, usize) = (0, 0);
        for (i, row) in rows.enumerate() {
            let line: Vec<char> = row.chars().collect();
            if line.contains(&'S') {
                start_pos = (i, line.iter().position(|&x| x == 'S').unwrap())
            }
            field.push(line);
        }
        let new_directions = hashmap! {
            'N' => hashmap! {
                '7' => 'W',
                '|' => 'N',
                'F' => 'E',
            },
            'E' => hashmap! {
                '7' => 'S',
                '-' => 'E',
                'J' => 'N',
            },
            'S' => hashmap! {
                'L' => 'E',
                '|' => 'S',
                'J' => 'W',
            },
            'W' => hashmap! {
                'L' => 'N',
                '-' => 'W',
                'F' => 'S',
            }
        };
        let mut directions: HashMap<char, (isize, isize)> = HashMap::new();
        directions.insert('N', (-1, 0));
        directions.insert('S', (1, 0));
        directions.insert('E', (0, 1));
        directions.insert('W', (0, -1));
        let mut current_pos: (usize, usize) = (start_pos.0, start_pos.1);
        let mut step: usize = 0;
        let mut area: isize = 0;
        let mut direction: char = 'N';
        if field[start_pos.0][start_pos.1 + 1] == '7' || field[start_pos.0][start_pos.1 + 1] == '-' || field[start_pos.0][start_pos.1 + 1] == 'J' {
            direction = 'E';
        }
        if field[start_pos.0][start_pos.1 - 1] == 'L' || field[start_pos.0][start_pos.1 - 1] == '-' || field[start_pos.0][start_pos.1 - 1] == 'F' {
            direction = 'W';
        }
        while current_pos != start_pos || step == 0 {
            step += 1;
            current_pos = (
                (current_pos.0 as isize + directions[&direction].0) as usize,
                (current_pos.1 as isize + directions[&direction].1) as usize,
            );
            println!("{} {} {}", area, current_pos.0, directions[&direction].1);
            area += current_pos.0 as isize * directions[&direction].1;
            direction = *new_directions
                .get(&direction).unwrap()
                .get(&field[current_pos.0][current_pos.1]).unwrap_or(&'S')
        }
        format!("{}", area as usize - step / 2 + 1)
    }
}
