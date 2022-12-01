pub mod day1;

fn main() {
    let day1data: Vec<Vec<usize>> = day1::read_data("day1.txt");
    let mostcalories: usize = day1::get_max(&day1data);
    let topncalories: usize = day1::get_max_n(&day1data, 3);
    println!("Answer to Day 1, Puzzle 1: {}", mostcalories);
    println!("Answer to Day 1, Puzzle 2: {}", topncalories);

}
