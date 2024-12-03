mod aoc;
mod d01;
mod d02;
mod d03;

use std::fs;

fn main() {
    solve_day(1);
}

//inputs are stored in root directory as dataXX.txt
fn solve_day(day: i32) {
    let data = fs::read_to_string(format!("data{day:0>2}.txt")).unwrap();
    match day {
        1 => d01::solve(data),
        2 => d02::solve(data),
        3 => d03::solve(data),
        _ => {}
    }
}