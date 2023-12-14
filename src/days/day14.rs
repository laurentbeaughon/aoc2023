use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayFourteen {}


fn tilt_north(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..map[0].len() {
        let mut pos_pushed: usize = 0;
        for j in 0..map.len() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[pos_pushed][i] = 'O';
                pos_pushed += 1;
            }
            if map[j][i] == '#' {
                pos_pushed = j + 1;
            }
        }
    }
    map
}

fn tilt_south(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for i in 0..map[0].len() {
        let mut pos_pushed: usize =  map.len();
        for j in (0..map.len()).rev() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[pos_pushed - 1][i] = 'O';
                pos_pushed -= 1;
            }
            if map[j][i] == '#' {
                pos_pushed = j;
            }
        }
    }
    map
}

fn tilt_west(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for j in 0..map.len() {
        let mut pos_pushed: usize = 0;
        for i in 0..map[0].len() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[j][pos_pushed] = 'O';
                pos_pushed += 1;
            }
            if map[j][i] == '#' {
                pos_pushed = i + 1;
            }
        }
    }
    map
}

fn tilt_east(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for j in 0..map.len() {
        let mut pos_pushed: usize = map[0].len();
        for i in (0..map[0].len()).rev() {
            if map[j][i] == 'O' {
                map[j][i] = '.';
                map[j][pos_pushed - 1] = 'O';
                pos_pushed -= 1;
            }
            if map[j][i] == '#' {
                pos_pushed = i;
            }
        }
    }
    map
}

fn count_load(map: Vec<Vec<char>>) -> usize {
    let mut result: usize = 0;
    let length = map.len();
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            if map[j][i] == 'O' {
                result += length - j;
            }
        }
    }
    result
}

impl AOCDay for DayFourteen {
    fn part_one(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = input.lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        map = tilt_north(map);
        let result: usize = count_load(map);
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let mut map: Vec<Vec<char>> = input.lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.chars().collect())
            .collect();
        let mut i: usize = 0;
        let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
        let mut cycle: usize = 0;
        let mut start_cycle: usize = 0;
        while i < 1_000_000_000 {
            map = tilt_north(map);
            map = tilt_west(map);
            map = tilt_south(map);
            map = tilt_east(map);
            if seen.contains_key(&map) {
                i += 1;
                start_cycle = *seen.get(&map).unwrap();
                cycle = i - start_cycle;
                i += 1_000_000_000;
            }
            else {
                i += 1;
                seen.insert(map.clone(), i);
            };
        }
        for _ in 0..(1_000_000_000 - start_cycle) % cycle {
            map = tilt_north(map);
            map = tilt_west(map);
            map = tilt_south(map);
            map = tilt_east(map);
        }

        let result: usize = count_load(map);
        format!("{}", result)
    }
}