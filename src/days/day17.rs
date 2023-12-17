use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;

use super::aocday::AOCDay;

pub struct DaySeventeen {}


fn find_path(grid: Vec<Vec<u8>>, min_length_dir: isize, max_length_dir: isize) -> usize {
    let mut visited: HashMap<((usize, usize), (isize, isize)), usize> = HashMap::new();
    let mut state: BinaryHeap<Reverse<(usize, (usize, usize), (isize, isize))>> = BinaryHeap::new();
    state.push(Reverse((0, (0, 0), (0, 0))));
    let directions: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    while let Some(tuple) = state.pop() {
        let heat = tuple.0.0;
        let position = tuple.0.1;
        let direction = tuple.0.2;
        if position.0 == grid.len() - 1 && position.1 == grid[0].len() - 1 {
            return heat;
        }
        for new_direction in &directions {
            if (new_direction.0 == -direction.0 && new_direction.1 == -direction.1) || (new_direction.0 == direction.0 && new_direction.1 == direction.1)  {
              continue;
            }
            let mut new_heat = heat;
            for length_dir in 1..max_length_dir + 1 {
                let new_position: (usize, usize) = (
                    (position.0 as isize + length_dir * new_direction.0) as usize,
                    (position.1 as isize + length_dir * new_direction.1) as usize,
                );
                if new_position.0 >= grid.len() || new_position.1 >= grid[0].len() {
                   continue;
                }
                new_heat += (grid[new_position.0][new_position.1]) as usize;
                if min_length_dir <= length_dir && new_heat < *visited.get(&(new_position, *new_direction)).unwrap_or(&10000000) {
                    visited.insert((new_position, *new_direction), new_heat);
                    state.push(Reverse((new_heat, new_position, *new_direction)));
                }
            }
       }
    }
    0
}

impl AOCDay for DaySeventeen {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let grid: Vec<Vec<u8>> = rows
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let result: usize = find_path(grid, 1, 3);
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let grid: Vec<Vec<u8>> = rows
            .map(|row| row.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect();
        let result: usize = find_path(grid, 4, 10);
        format!("{}", result)
    }
}