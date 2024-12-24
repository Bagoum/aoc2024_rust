use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use itertools::Itertools;

pub fn solve(data: String) {
    let mut to_idx = HashMap::new();
    let mut to_str = vec![];
    let mut adjacent = vec![];
    for p in data.lines() {
        let split = p.split('-').collect_vec();
        let a = match to_idx.entry(split[0]) {
            Entry::Vacant(v) => {
                to_str.push(split[0]);
                adjacent.push(HashSet::new());
                *v.insert(to_str.len() - 1)
            }, Entry::Occupied(v) => *v.get()
        };
        let b = match to_idx.entry(split[1]) {
            Entry::Vacant(v) => {
                to_str.push(split[1]);
                adjacent.push(HashSet::new());
                *v.insert(to_str.len() - 1)
            }, Entry::Occupied(v) => *v.get()
        };
        adjacent[a].insert(b);
        adjacent[b].insert(a);
    }
    let mut max_network = vec![vec![]];
    for i in 1.. {
        let next_max = max_network.iter().flat_map(|nw| try_expand(nw, &adjacent)).collect_vec();
        if i == 3 {
            println!("Part 1 solution: {}",
                     next_max.iter().filter(|w| w.iter().any(|x| to_str[*x].starts_with('t'))).count());
        } else if next_max.is_empty() {
            println!("Part 2 solution: {}",
                     max_network[0].iter().map(|x| to_str[*x]).sorted().join(","));
            break;
        }
        max_network = next_max;
    }
}

fn try_expand(clique: &Vec<usize>, adjacent: &Vec<HashSet<usize>>) -> Vec<Vec<usize>> {
    let mut results = vec![];
    for k in match clique.last() { None => 0, Some(x) => x + 1 }..adjacent.len() {
        let kadj = &adjacent[k];
        if clique.iter().any(|v| !kadj.contains(v)) { continue; }
        let mut result = clique.clone();
        result.push(k);
        results.push(result);
    }
    results
}