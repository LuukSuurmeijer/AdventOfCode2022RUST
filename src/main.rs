pub mod day1;
pub mod day2;
pub mod utils;
use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    if day == "1" {
        let day1data: Vec<Vec<usize>> = day1::read_data("day1.txt");
        let mostcalories: usize = day1::get_max(&day1data);
        let topncalories: usize = day1::get_max_n(&day1data, 3);
        println!("Answer to Day 1, Puzzle 1: {}\n", mostcalories);
        println!("Answer to Day 1, Puzzle 2: {}\n", topncalories);
    }

    else if day == "2" {
        let day2data = utils::read_to_string("day2.txt");
        let score: usize = day2::get_scores(day2data, false);
        print!("Answer to Day 2, Puzzle 1: {}\n", score);
        let day2data = utils::read_to_string("day2.txt");
        let score: usize = day2::get_scores(day2data, true);
        print!("Answer to Day 2, Puzzle 2: {}\n", score);
    }

}