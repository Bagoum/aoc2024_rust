use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(data: String) {
    let lines = data.lines().collect_vec();
    let height = lines.len() as i64;
    let width = lines[0].len() as i64;
    let mut locations = HashMap::new();
    for (ir,l) in lines.iter().enumerate() {
        for (ic,c) in l.chars().enumerate() {
            if c != '.' {
                locations.entry(c).or_insert(HashSet::new()).insert((ic as i64, ir as i64));
            }
        }
    }
    let in_bounds = |x,y| (x >= 0 && x < width && y >= 0 && y < height).then_some((x,y));
    let find_antinodes = |&(ax,ay), &(bx,by), allow_all: bool| if !allow_all {
        vec![
            in_bounds(ax + 2 * (bx-ax), ay + 2 * (by-ay)),
            in_bounds(ax - 1 * (bx-ax), ay - 1 * (by-ay))
        ].into_iter().filter_map(|x| x).collect_vec()
    } else {
        (0..).map_while(|i| in_bounds(ax+i*(bx-ax), ay+i*(by-ay))).chain(
            (1..).map_while(|i| in_bounds(ax-i*(bx-ax), ay-i*(by-ay)))).collect_vec()
    };
    let find_all_antinodes = |allow_all| locations.iter().flat_map(|(_, locs)| {
        let locs = locs.iter().collect_vec();
        return (0..locs.len()).flat_map(|ii|
                ((ii+1)..locs.len()).flat_map(|jj|
                    find_antinodes(&locs[ii], &locs[jj], allow_all)).collect_vec()
        ).collect_vec()
    }).unique().count();

    println!("Part 1 solution: {}", find_all_antinodes(false));
    println!("Part 2 solution: {}", find_all_antinodes(true));
}

