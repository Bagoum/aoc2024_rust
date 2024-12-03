use itertools::Itertools;
use regex::Regex;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let entries = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|do(n't)?\(\)")
        .unwrap().captures_iter(&data).collect_vec();
    let solver = |use_enablers: bool| entries.iter().fold((0, true), |(sum, enabled), c|
        match (c.get(2).zip(c.get(3)), c.get(4)) {
            (Some((left, right)), _) if enabled =>
                (sum + left.toi64() * right.toi64(), enabled),
            (None, cmd) => (sum, cmd.is_none() || !use_enablers),
            _ => (sum, enabled)
        }).0;

    println!("Part 1 solution: {}", solver(false));
    println!("Part 2 solution: {}", solver(true));
}
