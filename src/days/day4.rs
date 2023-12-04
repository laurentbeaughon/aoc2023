use super::aocday::AOCDay;

pub struct DayFour {}

impl AOCDay for DayFour {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split("\n");
        let mut total_score: i32 = 0;
        for row in rows {
            let game: Vec<&str> = row.split(": ").collect();
            let splitted_game: Vec<&str> = game[1].split(" | ").collect();
            let cards: Vec<i32> = splitted_game[0]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let reference: Vec<i32> = splitted_game[1]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let mut score: i32 = 0;
            for card in cards {
                if reference.contains(&card) {
                    score += 1;
                }
            }
            if score > 0 {
                total_score += 2_i32.pow((score - 1).try_into().unwrap());
            }
        }
    format!("{}", total_score)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split("\n");
        let mut final_cards: Vec<usize> = vec![1; 198];
        let mut i: usize = 0;
        for row in rows {
            let game: Vec<&str> = row.split(": ").collect();
            let splitted_game: Vec<&str> = game[1].split(" | ").collect();
            let cards: Vec<i32> = splitted_game[0]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let reference: Vec<i32> = splitted_game[1]
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .flat_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            let mut score: usize = 0;
            for card in cards {
                if reference.contains(&card) {
                    score += 1;
                }
            }
            let to_add: usize = final_cards[i];
            if score > 0 {
                for j in i+1..i+score+1 {
                    final_cards.get_mut(j).and_then(|value| {
                        *value += to_add;
                        Some(())
                    });
                }
            }
            i += 1;
        }
    format!("{}", final_cards.iter().sum::<usize>())
    }
}
