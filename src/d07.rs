use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let equations = data.lines().map(|l| {
        let mut nums = l.split_whitespace();
        let total = nums.next().unwrap();
        (total[..(total.len()-1)].toi64(), nums.map(|x|x.toi64()).collect_vec())
    }).collect_vec();
    let successful_sum = |concat| equations.iter().map(|(total, nums)|
        if can_total(*total, &nums, concat) { total } else { &0 }).sum::<i64>();
    println!("Part 1 solution: {}", successful_sum(false));
    println!("Part 2 solution: {}", successful_sum(true));
}

fn can_total(target: i64, options: &[i64], concat: bool) -> bool {
    if target < 0 { return false }
    if options.len() == 1 { return target == options[0] }
    let last = options[options.len()-1];
    let rest = &options[..options.len()-1];
    (target % last == 0 && can_total(target/last, rest, concat)) ||
        can_total(target-last, rest, concat) || concat && {
            let div = 10i64.pow(last.max(1).ilog10() + 1);
            (target-last) % div == 0 && can_total((target-last)/div, rest, concat)
        }
}