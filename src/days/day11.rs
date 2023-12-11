use super::aocday::AOCDay;

pub struct DayEleven {}


impl AOCDay for DayEleven {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut image: Vec<Vec<char>> = Vec::new();
        let mut galaxies: Vec<(usize, usize)> = Vec::new();
        let mut width_i: Vec<usize> = Vec::new();
        let mut width_j: Vec<usize> = Vec::new();
        for (i, row) in rows.enumerate() {
            let line: Vec<char> = row.chars().collect();
            if line.contains(&'#') {
                let indexes: Vec<usize> = line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == '#')
                    .map(|(index, _)| index)
                    .collect();
                for index in indexes {
                    galaxies.push((i, index));
                }
                width_i.push(1);
            }
            else {
                width_i.push(2);
            }
            image.push(line);
        }
        for i in 0..image[0].len() {
            if galaxies.iter().any(|&(_, value)| value == i) {
                width_j.push(1);
            }
            else {
                width_j.push(2);
            }
        }
        for i in &galaxies {
            println!("{}, {}", i.0, i.1);
        }
        let mut result: usize = 0;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let mut distance: usize = 0;
                if galaxies[i].0 > galaxies[j].0 {
                    for index in galaxies[j].0..galaxies[i].0 {
                        distance += width_i[index];
                    }
                }
                else {
                    for index in galaxies[i].0..galaxies[j].0 {
                        distance += width_i[index];
                    }
                }
                if galaxies[i].1 > galaxies[j].1 {
                    for index in galaxies[j].1..galaxies[i].1 {
                        distance += width_j[index];
                    }
                }
                else {
                    for index in galaxies[i].1..galaxies[j].1 {
                        distance += width_j[index];
                    }
                }
                result += distance;
            }
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut image: Vec<Vec<char>> = Vec::new();
        let mut galaxies: Vec<(usize, usize)> = Vec::new();
        let mut width_i: Vec<usize> = Vec::new();
        let mut width_j: Vec<usize> = Vec::new();
        for (i, row) in rows.enumerate() {
            let line: Vec<char> = row.chars().collect();
            if line.contains(&'#') {
                let indexes: Vec<usize> = line.iter()
                    .enumerate()
                    .filter(|&(_, &c)| c == '#')
                    .map(|(index, _)| index)
                    .collect();
                for index in indexes {
                    galaxies.push((i, index));
                }
                width_i.push(1);
            }
            else {
                width_i.push(1000000);
            }
            image.push(line);
        }
        for i in 0..image[0].len() {
            if galaxies.iter().any(|&(_, value)| value == i) {
                width_j.push(1);
            }
            else {
                width_j.push(1000000);
            }
        }
        for i in &galaxies {
            println!("{}, {}", i.0, i.1);
        }
        let mut result: usize = 0;
        for i in 0..galaxies.len() {
            for j in (i + 1)..galaxies.len() {
                let mut distance: usize = 0;
                if galaxies[i].0 > galaxies[j].0 {
                    for index in galaxies[j].0..galaxies[i].0 {
                        distance += width_i[index];
                    }
                }
                else {
                    for index in galaxies[i].0..galaxies[j].0 {
                        distance += width_i[index];
                    }
                }
                if galaxies[i].1 > galaxies[j].1 {
                    for index in galaxies[j].1..galaxies[i].1 {
                        distance += width_j[index];
                    }
                }
                else {
                    for index in galaxies[i].1..galaxies[j].1 {
                        distance += width_j[index];
                    }
                }
                result += distance;
            }
        }
        format!("{}", result)
    }
}
