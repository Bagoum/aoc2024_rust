use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub fn solve(data: String) {
    let mut pq = PriorityQueue::new();
    //(loc,dir) -> (best_score, Set<prev_locs>)
    let mut visited = HashMap::new();
    let matrix = data.lines().enumerate().map(|(y,l)|
        l.chars().enumerate().map(|(x,c)| match c {
            'S' => { pq.push(((x as i64,y as i64,1,0),None), 0); '.' }
            c => c
        }).collect_vec()).collect_vec();
    let ((mut endx, mut endy), mut final_score) = ((0, 0), i32::MIN);
    while let Some(((loc@(x,y,dx,dy),prevloc), score)) = pq.pop() {
        //Since we need to get all best paths-to-sink, we can only stop iterating once
        // the next path score in the priority queue is worse than the best score-to-sink
        if score < final_score { break; }
        match matrix[y as usize][x as usize] {
            'E' => ((endx, endy), final_score) = ((x, y), score),
            '#' => continue,
            _ => {},
        }
        match visited.entry(loc) {
            Entry::Vacant(v) => { v.insert((score, HashSet::from([prevloc]))); }
            Entry::Occupied(v) => {
                let (best_score, possible_prev) = v.into_mut();
                if score == *best_score {
                    possible_prev.insert(prevloc);
                } //alternate path to same point; add to possible_prev but don't re-expand node
                continue;
            }
        }
        //Rust PQ pulls highest priority first, so invert scores in order to minimize
        pq.push(((x+dx,y+dy,dx,dy), Some(loc)),score-1);
        pq.push(((x,y,dy,dx), Some(loc)), score-1000);
        pq.push(((x,y,-dy,-dx), Some(loc)), score-1000);
    }
    println!("Part 1 solution: {}", -final_score);
    let mut best_paths = HashSet::new();
    let mut backtracker = vec![(endx,endy,1i64,0i64),(endx,endy,0,1),(endx,endy,-1,0),(endx,endy,0,-1)];
    while let Some(nxt) = backtracker.pop() {
        if best_paths.insert(nxt) {
            visited.get(&nxt).map(|(_, prevs)| prevs.into_iter().filter_map(|x| *x)
                                                        .for_each(|x| backtracker.push(x)));
        }
    }
    println!("Part 2 solution: {}", best_paths.into_iter().map(|(x,y,_,_)|(x,y)).unique().count());

}