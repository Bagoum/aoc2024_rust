mod aoc;
mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;

use std::fs;

fn main() {
    solve_day(11);
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
        8 => d08::solve(data),
        9 => d09::solve(data.as_str()),
        10 => d10::solve(data),
        11 => d11::solve(data),
        _ => {}
    }
}