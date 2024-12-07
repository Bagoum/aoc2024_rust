use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::aoc::{ToI64, Vec2};

pub fn solve(data: String) {
    let mut start_loc = (0, 0);
    let mut matrix = data.lines().enumerate().map(|(y, l)| l.chars().enumerate().map(|(x, c)| {
        if c == '^' { start_loc = (x.toi64(), y.toi64()); }
        return c == '#'
    }).collect_vec()).collect_vec();
    let nsteps = |matrix: &Vec<Vec<bool>>| {
        let mut curr_loc = start_loc;
        let mut curr_dir = (0i64, -1i64);
        let mut visited = HashMap::new();
        while matrix.indexp(curr_loc).is_some() {
            if !visited.entry(curr_loc).or_insert(HashSet::new()).insert(curr_dir) {
                return None;
            }
            let next_loc = (curr_loc.0 + curr_dir.0, curr_loc.1 + curr_dir.1);
            match matrix.indexp(next_loc) {
                Some(true) => { curr_dir = (-1 * curr_dir.1, curr_dir.0) },
                _ => { curr_loc = next_loc }
            }
        }
        return Some(visited);
    };
    let normal_visited = nsteps(&matrix).unwrap();
    println!("Part 1 solution: {}", normal_visited.len());

    let infinites: usize = normal_visited.keys().filter(|(x,y)| {
        let (x,y) = (*x as usize, *y as usize);
        if matrix[y][x] { return false; }
        matrix[y][x] = true;
        let result = nsteps(&matrix).is_none();
        matrix[y][x] = false;
        return result;
    }).count();
    println!("Part 2 solution: {}", infinites);
}
