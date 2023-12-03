use std::env;
use std::fs;

mod days {
    pub mod aocday;
    pub mod day1;
    pub mod day2;
    pub mod day3;
}
use crate::days::aocday::AOCDay;
use crate::days::day1::DayOne;
use crate::days::day2::DayTwo;
use crate::days::day3::DayThree;

fn day_to_problem(day: u8) -> Option<Box<dyn AOCDay>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{})),
        3 => Some(Box::new(DayThree{})),
        // ...
        _ => None
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }
    
    let day: u8 = match args[1].parse() {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Invalid day argument. Please provide a valid day as a number.");
            std::process::exit(1);
        }
    };
    let file = match fs::read_to_string(format!("data/day{day}.txt")){
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let problem = day_to_problem(day);

    //problem.part_one ??
    let output1 = problem.as_ref().map_or_else(
        || "Invalid day or not implemented.".to_string(),
        |p| p.part_one(&file),
    );
    let output2 = problem.as_ref().map_or_else(
        || "Invalid day or not implemented.".to_string(),
        |p| p.part_two(&file),
    );

    println!("{output1}, {output2}");
}