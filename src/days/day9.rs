use super::aocday::AOCDay;

pub struct DayNine {}

fn next_value(list: Vec<i32>) -> (i32, i32) {
    if list.iter().all(|&x| x == list[0]) {
        return (list[0], list[0]);
    }
    else {
        let mut new_vec: Vec<i32> = Vec::new();
        for i in 1..list.len() {
            new_vec.push(list[i] - list[i-1]);
        }
        let next_values: (i32, i32) = next_value(new_vec);
        return (
            list[0] - next_values.0,
            list.iter().last().unwrap() + next_values.1
        );
    }
}

impl AOCDay for DayNine {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut result: i32 = 0;
        for row in rows {
            let values = row
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            result += next_value(values).1;
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut result: i32 = 0;
        for row in rows {
            let values = row
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            result += next_value(values).0;
        }
        format!("{}", result)
    }
}
