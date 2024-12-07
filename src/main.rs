mod aoc;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;

use std::fs;

fn main() {
    solve_day(7);
}

//inputs are stored in root directory as dataXX.txt
fn solve_day(day: i32) {
    let data = fs::read_to_string(format!("data{day:0>2}.txt")).unwrap();
    match day {
        1 => d01::solve(data),
        2 => d02::solve(data),
        3 => d03::solve(data),
        4 => d04::solve(data),
        5 => d05::solve(data),
        6 => d06::solve(data),
        7 => d07::solve(data),
        _ => {}
    }
}