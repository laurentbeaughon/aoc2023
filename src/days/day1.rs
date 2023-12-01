use regex::Regex;

use super::aocday::AOCDay;

pub struct DayOne {}

fn match_to_digit(digit: &str) -> u32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        // ...
        _ => 0
    }
}

impl AOCDay for DayOne {
    fn part_one(&self, input: &str) -> String {
        let first_regex = Regex::new(r"^[a-z]*\d").expect("failed to compile regex");
        let last_regex = Regex::new(r"\d[a-z]*$").expect("failed to compile regex");
        let rows = input.split("\n");
        let mut total: u32 = 0;
        for row in rows {
            let first = first_regex.find(row).map(|x| x.as_str()).unwrap().chars().collect::<Vec<char>>();
            let first_digit: u32 = first[first.len() - 1].to_digit(10).unwrap();
            let end = last_regex.find(row).map(|x| x.as_str()).unwrap().chars().collect::<Vec<char>>()[0];
            let end_digit: u32 = end.to_digit(10).unwrap();
            total += 10*first_digit + end_digit;
        }
        format!("{total}")
    }

    fn part_two(&self, input: &str) -> String {
        let first_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").expect("failed to compile regex");
        let last_regex = Regex::new(r"(?:.*)(one|two|three|four|five|six|seven|eight|nine|\d)").expect("failed to compile regex");
        let rows = input.split("\n");
        let mut total: u32 = 0;
        for row in rows {
            let first = first_regex.captures(row).unwrap().get(0).unwrap().as_str();
            let first_digit: u32 = match_to_digit(first);
            let last = last_regex.captures(row).unwrap().get(1).unwrap().as_str();
            let last_digit: u32 = match_to_digit(last);
            total += 10*first_digit + last_digit;
        }
        format!("{total}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}