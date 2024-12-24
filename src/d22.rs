use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let nums = data.lines().map(|l| l.toi64()).collect_vec();
    let mut total_sellat = HashMap::new();
    let mut total_2000th = 0;
    for x0 in nums {
        let mut local_sellat = HashSet::new();
        let mut a = step(x0);
        let mut b = step(a);
        let mut d1;
        let (mut d2, mut d3) = ((a%10)-(x0%10), (b%10)-(a%10));
        (a,b) = (b, step(b));
        let mut d4 = (b%10)-(a%10);
        for _ in 3..2000 {
            (a, b) = (b, step(b));
            (d1, d2, d3, d4) = (d2, d3, d4, (b%10)-(a%10));
            if local_sellat.insert((d1,d2,d3,d4)) {
                match total_sellat.entry((d1,d2,d3,d4)) {
                    Entry::Occupied(v) => *v.into_mut() += b % 10,
                    Entry::Vacant(v) => { v.insert(b % 10); },
                };
            }
        }
        total_2000th += b;
    }
    println!("Part 1 solution: {total_2000th}");
    println!("Part 2 solution: {}", total_sellat.into_iter().map(|(_,v)| v).max().unwrap());
}
fn step(x: i64) -> i64 {
    let x1 = ((x * 64) ^ x) % 16777216;
    let x2 = ((x1 / 32) ^ x1) % 16777216;
    ((x2 * 2048) ^ x2) % 16777216
}