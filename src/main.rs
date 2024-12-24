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
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;

use std::fs;

fn main() {
    solve_day(24);
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
        12 => d12::solve(data),
        13 => d13::solve(data),
        14 => d14::solve(data),
        15 => d15::solve(data),
        16 => d16::solve(data),
        17 => d17::solve(data),
        18 => d18::solve(data),
        19 => d19::solve(data),
        20 => d20::solve(data),
        21 => d21::solve(data),
        22 => d22::solve(data),
        23 => d23::solve(data),
        24 => d24::solve(data),
        _ => {}
    }
}