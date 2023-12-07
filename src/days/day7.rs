use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DaySeven {}

fn hand_score(game: Vec<char>) -> Vec<i8> {
    let mut score: Vec<i8> = Vec::new();
    let order: Vec<char> = vec!['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let mut char_counts: HashMap<char, i8> = HashMap::new();
    for &c in &game {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let mut sorted_counts: Vec<(char, i8)> = char_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
    let hand_type: i8 = match sorted_counts.as_slice() {
        [(_, count1)] if *count1 == 5 => 6,
        [(_, count1), (_, _)] if *count1 == 4 => 5,
        [(_, count1), (_, count2)] if *count1 == 3 && *count2 == 2 => 4,
        [(_, count1), (_, _), (_, _)] if *count1 == 3 => 3,
        [(_, count1), (_, count2), (_, _)] if *count1 == 2 && *count2 == 2 => 2,
        [(_, count1), (_, _), (_, _), (_, _)] if *count1 == 2 => 1,
        _ => 0,
    };
    score.push(hand_type);
    for c in game {
        score.push(
            order
            .iter().position(|&x| x == c)
            .map(|index| index as i8)
            .unwrap()
        )
    }
    score
}

fn hand_score_2(game: Vec<char>) -> Vec<i8> {
    let mut score: Vec<i8> = Vec::new();
    let order: Vec<char> = vec!['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];
    let mut char_counts: HashMap<char, i8> = HashMap::new();
    char_counts.insert('J', 0);
    for &c in &game {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    let mut sorted_counts: Vec<(char, i8)> = char_counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
    if sorted_counts[0].0 == 'J' && sorted_counts[0].1 == 5 {}
    else if sorted_counts[0].0 == 'J' {
        sorted_counts[1].1 += sorted_counts[0].1;
        sorted_counts.remove(0);
    }
    else {
        let index_j: usize = sorted_counts.iter().position(|&x| x.0 == 'J').unwrap();
        sorted_counts[0].1 += sorted_counts[index_j].1;
        sorted_counts.remove(index_j);
    }
    let hand_type: i8 = match sorted_counts.as_slice() {
        [(_, count1)] if *count1 == 5 => 6,
        [(_, count1), (_, _)] if *count1 == 4 => 5,
        [(_, count1), (_, count2)] if *count1 == 3 && *count2 == 2 => 4,
        [(_, count1), (_, _), (_, _)] if *count1 == 3 => 3,
        [(_, count1), (_, count2), (_, _)] if *count1 == 2 && *count2 == 2 => 2,
        [(_, count1), (_, _), (_, _), (_, _)] if *count1 == 2 => 1,
        _ => 0,
    };
    score.push(hand_type);
    for c in game {
        score.push(
            order
            .iter().position(|&x| x == c)
            .map(|index| index as i8)
            .unwrap()
        )
    }
    score
}

impl AOCDay for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut scores: Vec<(Vec<i8>, usize)> = Vec::new();
        for row in rows {
            let data: Vec<&str> = row.split_whitespace().collect::<Vec<&str>>();
            let hand: Vec<char> = data[0].chars().collect();
            let bid: usize = data[1].parse::<usize>().unwrap();
            let score: Vec<i8> = hand_score(hand);
            scores.push((score, bid));
        }
        scores.sort_by(|(a, _), (b, _)| {
            a.iter().cmp(b.iter())
        });
        let mut result: usize = 0;
        for (i, (_, value)) in scores.iter().enumerate() {
            result += (i + 1) * value;
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let mut scores: Vec<(Vec<i8>, usize)> = Vec::new();
        for row in rows {
            let data: Vec<&str> = row.split_whitespace().collect::<Vec<&str>>();
            let hand: Vec<char> = data[0].chars().collect();
            let bid: usize = data[1].parse::<usize>().unwrap();
            let score: Vec<i8> = hand_score_2(hand);
            scores.push((score, bid));
        }
        scores.sort_by(|(a, _), (b, _)| {
            a.iter().cmp(b.iter())
        });
        let mut result: usize = 0;
        for (i, (_, value)) in scores.iter().enumerate() {
            result += (i + 1) * value;
        }
        format!("{}", result)
    }
}
