use std::collections::HashMap;
use std::collections::hash_map::Entry;
use itertools::Itertools;

pub fn solve(data: String) {
    let mut lines = data.lines();
    //You don't actually need a trie for this problem; iterating over a vector suffices.
    let mut towels = Trie::new();
    lines.next().unwrap().split(", ").into_iter().for_each(|towel| towels.add(towel));
    lines.next(); //blank line
    let mut cache = HashMap::new();
    let matches = lines.map(|l| multimatches(&towels, l, &mut cache)).collect_vec();
    println!("Part 1 solution: {}", matches.iter().filter(|x| **x > 0).count());
    println!("Part 2 solution: {}", matches.iter().sum::<usize>());
}

fn multimatches<'a>(towels: &Trie, sequence: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    if sequence.is_empty() {
        return 1;
    } else if let Some(cached) = cache.get(sequence) {
        return *cached
    }
    let matches = towels.match_prefix(sequence);
    let result = matches.into_iter().map(|x| multimatches(towels, &sequence[x..], cache)).sum();
    cache.insert(sequence, result);
    result
}

struct Trie {
    is_leaf: bool,
    branches: HashMap<char, Trie>
}
impl Trie {
    fn new() -> Trie {
        Trie {
            is_leaf: false,
            branches: HashMap::new()
        }
    }

    fn match_prefix(&self, str: &str) -> Vec<usize> {
        let mut results = vec![];
        if self.is_leaf {
            results.push(0);
        }
        if let Some(c) = str.chars().next() {
            match self.branches.get(&c) {
                None => {},
                Some(branch) => {
                    for result in branch.match_prefix(&str[1..]).into_iter() {
                        results.push(result + 1);
                    }
                }
            }
        }
        results
    }

    fn add(&mut self, str: &str) {
        match str.chars().next() {
            None => self.is_leaf = true,
            Some(c) => match self.branches.entry(c) {
                Entry::Vacant(v) => v.insert(Trie::new()),
                Entry::Occupied(v) => v.into_mut()
            }.add(&str[1..])
        }
    }
}