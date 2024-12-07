use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let mut outgoing = HashMap::new();
    let mut orderings = vec![];
    for l in data.lines() {
        if l.contains('|') {
            let nums = l.split('|').collect_vec();
            outgoing.entry(nums[0].toi64()).or_insert(HashSet::new()).insert(nums[1].toi64());
        } else if l.contains(',') {
            orderings.push(l.split(",").map(|x| x.toi64()).collect_vec());
        }
    }
    let (correct, incorrect): (Vec<_>, Vec<_>) = orderings.into_iter().partition(|o|
        (1..o.len()).all(|ii|
            match outgoing.get(&o[ii]) {
                None => true,
                Some(adjacent) => !adjacent.contains(&o[ii - 1])
            }
    ));

    let correct_middle_sum: i64 = correct.iter().map(|o| &o[o.len()/2]).sum();
    println!("Part 1 solution: {correct_middle_sum}");

    let incorrect_middle_sum: i64 = incorrect.iter().map(|o| o.iter().find(|ii|
            match outgoing.get(ii) {
                None => false,
                Some(adjacent) => o.iter().filter(|jj| adjacent.contains(jj)).count() == o.len()/2
        }).unwrap()).sum();
    println!("Part 2 solution: {incorrect_middle_sum}");
}
