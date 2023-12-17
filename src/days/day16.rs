use maplit::hashmap;
use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DaySixteen {}


fn energized(grid: Vec<Vec<char>>, mut pos: (usize, usize), direction: char, mut visited: HashMap<(usize, usize), Vec<char>>) -> HashMap<(usize, usize), Vec<char>> {
    let directions: HashMap<char, (isize, isize)> = hashmap! {
        'n' => (-1, 0),
        's' => (1, 0),
        'e' => (0, 1),
        'w' => (0, -1),
    };

    let turns: HashMap<char, HashMap<char, Vec<char>>> = hashmap! {
        'n' => hashmap! {
            '/' => vec!['e'],
            '|' => vec!['n', 's'],
            '-' => vec!['e', 'w'],
            '\\' => vec!['w'],
        },
        'e' => hashmap! {
            '/' => vec!['n'],
            '|' => vec!['n', 's'],
            '-' => vec!['e', 'w'],
            '\\' => vec!['s'],
        },
        's' => hashmap! {
            '/' => vec!['w'],
            '|' => vec!['n', 's'],
            '-' => vec!['e', 'w'],
            '\\' => vec!['e'],
        },
        'w' => hashmap! {
            '/' => vec!['s'],
            '|' => vec!['n', 's'],
            '-' => vec!['e', 'w'],
            '\\' => vec!['n'],
        },
    };
    while pos.0 > 0 && pos.0 <= grid.len() && pos.1 > 0 && pos.1 <= grid[0].len() {
        let mut visit: Vec<char> = visited.entry(pos).or_default().clone();
        if visit.contains(&direction) {
            return visited;
        }
        visit.push(direction);
        visited.insert(pos, visit);
        let value: char = grid[pos.0 - 1][pos.1 - 1];
        if value != '.' {
            let new_directions: Vec<char> = turns.get(&direction).unwrap().get(&value).unwrap().clone();
            for new_direction in new_directions.iter() {
                let new_pos: (usize, usize) = (
                    (pos.0 as isize + directions[&new_direction].0) as usize,
                    (pos.1 as isize + directions[&new_direction].1) as usize,
                );
                visited = energized(grid.clone(), new_pos, *new_direction, visited);
            }
        }
        else {
            pos = (
                (pos.0 as isize + directions[&direction].0) as usize,
                (pos.1 as isize + directions[&direction].1) as usize,
            );
        }
    }
    visited
}


impl AOCDay for DaySixteen {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let grid: Vec<Vec<char>> = rows.into_iter()
            .map(|s| s.chars().collect())
            .collect();
        let visited: HashMap<(usize, usize), Vec<char>> = HashMap::new();

        let pos: (usize, usize) = (1, 1);
        let result: usize = energized(grid, pos, 'e', visited).len();

        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let grid: Vec<Vec<char>> = rows.into_iter()
            .map(|s| s.chars().collect())
            .collect();
            
        let mut best_result: usize = 0;
        for i in 1..grid.len() + 1 {
            println!("{i}");
            for j in 1..grid[0].len() + 1 {
                for direction in ['n', 'e', 's', 'w'] {
                    let visited: HashMap<(usize, usize), Vec<char>> = HashMap::new();
                    let result: usize = energized(grid.clone(), (i, j), direction, visited).len();
                    if result > best_result {
                        best_result = result;
                    }
                }
            }
        }
        format!("{}", best_result)
    }
}