use regex::Regex;
use std::collections::HashMap;


use super::aocday::AOCDay;

pub struct DayTwelve {}

#[derive(Debug, Eq, PartialEq, Hash)]
struct RecordsCriteria {
    records: Vec<char>,
    criteria: Vec<usize>,
}

fn count_arrangements(records: Vec<char>, criteria: Vec<usize>, mut cache: HashMap<RecordsCriteria, usize>) -> (usize, HashMap<RecordsCriteria, usize>) {
    if let Some(&val) = cache.get(&RecordsCriteria {records: records.clone(), criteria: criteria.clone()}) {
        return (val, cache);
    }
    if records.len() == 0 {
        if criteria.len() == 0 {
            return (1, cache);
        }
        else {
            return (0, cache);
        }
    }
    if criteria.len() == 0 {
        if records.contains(&'#') {
            return (0, cache)
        }
        else {
            return (1, cache);
        }
    }

    let mut result: usize = 0;

    if records[0] != '#' {
        let returned: (usize, HashMap<RecordsCriteria, usize>) = count_arrangements(records[1..].to_vec(), criteria.clone(), cache);
        result += returned.0;
        cache = returned.1;
    }
    let k: usize = criteria[0];
    if records[0] != '.' {
        if records.iter().take(k).all(|&c| c != '.') && k == records.len() && criteria.len() == 1 {
            return (1, cache)
        }
        if records.iter().take(k).all(|&c| c != '.') && k < records.len() && records[k] != '#' {
            let returned: (usize, HashMap<RecordsCriteria, usize>) = count_arrangements(records[(k + 1)..].to_vec(), criteria[1..].to_vec(), cache);
            result += returned.0;
            cache = returned.1;
        }
    }
    cache.insert(RecordsCriteria {records: records.clone(), criteria: criteria.clone()}, result);
    return (result, cache)

}

impl AOCDay for DayTwelve {
    fn part_one(&self, input: &str) -> String {
        let rows = input.split('\n');
        let regex = Regex::new(r"([\.\?#]+) ([0-9,]+)").expect("failed to compile regex");
        let mut result: usize = 0;
        let mut cache: HashMap<RecordsCriteria, usize> = HashMap::new();
        for row in rows {
            let records: Vec<char> = regex.captures(row).unwrap().get(1).unwrap().as_str().chars().collect();
            let criteria: Vec<usize> = regex
                .captures(row)
                .and_then(|captures| captures.get(2))
                .map(|capture| capture.as_str())
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();
            let returned: (usize, HashMap<RecordsCriteria, usize>) = count_arrangements(records, criteria, cache);
            result += returned.0;
            cache = returned.1;
        }
        format!("{}", result)
    }

    fn part_two(&self, input: &str) -> String {
        let rows = input.split('\n');
        let regex = Regex::new(r"([\.\?#]+) ([0-9,]+)").expect("failed to compile regex");
        let mut result: usize = 0;
        let mut cache: HashMap<RecordsCriteria, usize> = HashMap::new();
        for row in rows {
            let mut records: Vec<char> = regex.captures(row).unwrap().get(1).unwrap().as_str().chars().collect();
            records.push('?');
            let records5: Vec<char> = records.iter().cycle().take(records.len() * 5 - 1).cloned().collect();
            let criteria: Vec<usize> = regex
                .captures(row)
                .and_then(|captures| captures.get(2))
                .map(|capture| capture.as_str())
                .unwrap_or("")
                .split(',')
                .map(|s| s.trim().parse())
                .collect::<Result<Vec<usize>, _>>()
                .unwrap();
            let criteria5: Vec<usize> = criteria.iter().cycle().take(criteria.len() * 5).cloned().collect();
            let returned: (usize, HashMap<RecordsCriteria, usize>) = count_arrangements(records5, criteria5, cache);
            result += returned.0;
            cache = returned.1;
        }
        format!("{}", result)
    }
}