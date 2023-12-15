use std::collections::HashMap;

use super::aocday::AOCDay;

pub struct DayFifteen {}


fn hash_(word: &str) -> usize {
    let chars: Vec<char> = word.chars().collect();
    let mut hash: usize = 0;
    for c in chars {
        hash += c as usize;
        hash *= 17;
        hash = hash % 256;
    }
    hash
}

impl AOCDay for DayFifteen {
    fn part_one(&self, input: &str) -> String {
        let words = input.split(',');
        let mut result: usize = 0;
        for word in words {
            let hash: usize = hash_(word);
            result += hash;
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let words = input.split(',');
        let mut boxes: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
        for word in words {
            let chars: Vec<char> = word.chars().collect();
            if chars.contains(&'-') {
                let text: &str = &word[..word.len() - 1];
                let hash: usize = hash_(text);
                let mut tmp_vec: Vec<(&str, usize)> = (*boxes.entry(hash).or_insert(Vec::new())).to_vec();
                tmp_vec.retain(|&(key, _)| key != text);
                boxes.insert(hash, tmp_vec);
            }
            else {
                let i: usize = word.find('=').unwrap();
                let (text, value) = word.split_at(i);
                let value_int = value.trim_start_matches('=').parse::<usize>().unwrap();
                let hash: usize = hash_(text);
                let mut tmp_vec: Vec<(&str, usize)> = (*boxes.entry(hash).or_insert(Vec::new())).to_vec();
                if let Some(index) = tmp_vec.iter().position(|(key, _)| key == &text) {
                    tmp_vec[index] = (text, value_int);
                }
                else {
                    tmp_vec.push((text, value_int));
                }
                boxes.insert(hash, tmp_vec);
            }
        }
        let mut result: usize = 0;
        for (key, val) in boxes.iter() {
            for (i, tuple) in val.iter().enumerate() {
                result += (key + 1) * (i + 1) * tuple.1;
            }
        }
        format!("{}", result)
    }
}