use super::aocday::AOCDay;

pub struct DayThirteen {}


fn find_sym(rows: Vec<usize>, columns: Vec<usize>) -> usize {
    for i in 0..rows.len() - 1 {
        let mut is_sym: bool = true;
        for j in 0..i.min(rows.len() - i - 2) + 1 {
            if rows[i - j] != rows[i + j + 1] {
                is_sym = false;
            }
        }
        if is_sym {
            return 100 * (i + 1)
        }
    }
    for i in 0..columns.len() - 1 {
        let mut is_sym: bool = true;
        for j in 0..i.min(columns.len() - i - 2) + 1 {
            if columns[i - j] != columns[i + j + 1] {
                is_sym = false;
            }
        }
        if is_sym {
            return i + 1
        }
    }
    0
}

fn find_smog_sym(rows: Vec<usize>, columns: Vec<usize>) -> usize {
    for i in 0..rows.len() - 1 {
        let mut is_sym: i32 = 0;
        let mut diff: i32 = 0;
        for j in 0..i.min(rows.len() - i - 2) + 1 {
            if rows[i - j] != rows[i + j + 1] {
                for k in 0..columns.len() {
                    if (rows[i - j] + 3_i32.pow(k as u32) as usize) == rows[i + j + 1] || rows[i - j] == (3_i32.pow(k as u32) as usize + rows[i + j + 1]) {
                        diff += 1;
                    }
                }
                if diff == 1 {
                    is_sym += 1;
                    diff = 0;
                }
                else {
                    is_sym += 2;
                }
            }
        }
        if is_sym == 1 {
            return 100 * (i + 1)
        }
    }
    for i in 0..columns.len() - 1 {
        let mut is_sym: i32 = 0;
        let mut diff: i32 = 0;
        for j in 0..i.min(columns.len() - i - 2) + 1 {
            if columns[i - j] != columns[i + j + 1] {
                for k in 0..rows.len() {
                    if (columns[i - j] + 3_i32.pow(k as u32) as usize) == columns[i + j + 1] || columns[i - j] == (3_i32.pow(k as u32) as usize + columns[i + j + 1]) {
                        diff += 1;
                    }
                }
                if diff == 1 {
                    is_sym += 1;
                    diff = 0;
                }
                else {
                    is_sym += 2;
                }
            }
        }
        if is_sym == 1 {
            return i + 1
        }
    }
    0
}


impl AOCDay for DayThirteen {
    fn part_one(&self, input: &str) -> String {
        let maps = input.split("\n\n");
        let mut result: usize = 0;
        for map in maps {
            let mut rows = map.split('\n').peekable();
            let mut columns = vec![0; rows.peek().unwrap().len()];
            let mut rows_int = vec![0; rows.clone().count()];
            for (i, row) in rows.enumerate() {
                for (j, c) in row.chars().collect::<Vec<char>>().iter().enumerate() {
                    if c == &'#' {
                        columns[j] += 2_i32.pow(i as u32) as usize;
                        rows_int[i] += 2_i32.pow(j as u32) as usize;
                    }
                }
            }
            result += find_sym(rows_int, columns);
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let maps = input.split("\n\n");
        let mut result: usize = 0;
        for map in maps {
            let mut rows = map.split('\n').peekable();
            let mut columns = vec![0; rows.peek().unwrap().len()];
            let mut rows_int = vec![0; rows.clone().count()];
            for (i, row) in rows.enumerate() {
                for (j, c) in row.chars().collect::<Vec<char>>().iter().enumerate() {
                    if c == &'#' {
                        columns[j] += 3_i32.pow(i as u32) as usize;
                        rows_int[i] += 3_i32.pow(j as u32) as usize;
                    }
                }
            }
            result += find_smog_sym(rows_int, columns);
        }
        format!("{}", result)
    }
}