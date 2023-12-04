use regex::Regex;

use std::cmp;
use super::aocday::AOCDay;

pub struct DayTwo {}

fn is_valid(game: Vec<i32>) -> bool {
    game[0] <= 12 && game[1] <= 13 && game[2] <= 14
}

impl AOCDay for DayTwo {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let regex = Regex::new(r"(:)(.*)").expect("failed to compile regex");
        let regex_red = Regex::new(r"(\d+)( red)").expect("failed to compile regex");
        let regex_blue = Regex::new(r"(\d+)( blue)").expect("failed to compile regex");
        let regex_green = Regex::new(r"(\d+)( green)").expect("failed to compile regex");
        let mut sum_ids: i32 = 0;
        let mut i: i32 = 1;
        for row in rows {
            let games = regex.captures(row).unwrap().get(2).unwrap().as_str();
            let mut valid: bool = true;
            for game in games.split("; ").collect::<Vec<&str>>() {
                let red: i32 = regex_red.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                let green: i32 = regex_green.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                let blue: i32 = regex_blue.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                let game: Vec<i32> = vec![red, green, blue];
                valid = valid && is_valid(game);
            }
            if valid {
                sum_ids += i;
            }
            i += 1;
        }
        format!("{}", sum_ids)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let regex = Regex::new(r"(:)(.*)").expect("failed to compile regex");
        let regex_red = Regex::new(r"(\d+)( red)").expect("failed to compile regex");
        let regex_blue = Regex::new(r"(\d+)( blue)").expect("failed to compile regex");
        let regex_green = Regex::new(r"(\d+)( green)").expect("failed to compile regex");
        let mut sum_ids: i32 = 0;
        for row in rows {
            let games = regex.captures(row).unwrap().get(2).unwrap().as_str();
            println!("{}", games);
            let mut min_red: i32 = 0;
            let mut min_blue: i32 = 0;
            let mut min_green: i32 = 0;
            for game in games.split("; ").collect::<Vec<&str>>() {
                let red: i32 = regex_red.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                let green: i32 = regex_green.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                let blue: i32 = regex_blue.captures(game).map_or_else(
                    || "0".to_string(),
                    |captures| captures.get(1).map_or("0", |m| m.as_str()).to_string(),
                ).parse::<i32>().unwrap();
                println!("{} {} {}", red, green, blue);
                min_red = cmp::max(red, min_red);
                min_green = cmp::max(green, min_green);
                min_blue = cmp::max(blue, min_blue);
            }
            sum_ids += min_blue * min_green * min_red;
        }
        format!("{}", sum_ids)
    }
}
