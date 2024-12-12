use std::collections::HashMap;
use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let stonect = |rounds| {
        let (mut stones, mut nstones) = (data.split_whitespace().map(|x|x.toi64()).counts(), HashMap::new());
        for _ in 0..rounds {
            for (x, ct) in stones.into_iter() {
                let digits = x.max(1).ilog10() + 1;
                for nxt in match x {
                    0 => vec![1],
                    _ if digits % 2 == 0 => vec![x/10i64.pow(digits/2), x%10i64.pow(digits/2)],
                    _ => vec![x * 2024],
                } {
                    *nstones.entry(nxt).or_insert(0) += ct;
                }
            }
            (stones, nstones) = (nstones, HashMap::new());
        }
        return stones.iter().map(|(_,v)|v).sum::<usize>();
    };

    println!("Part 1 solution: {}", stonect(25));
    println!("Part 2 solution: {}", stonect(75));
}

