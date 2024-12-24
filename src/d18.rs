use std::collections::HashSet;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let nums = data.lines().map(|l| {
        let split = l.split(',').collect_vec();
        (split[0].toi64(), split[1].toi64())
    }).collect_vec();
    let first_k = |k| nums.iter().take(k).collect::<HashSet<_>>();
    println!("Part 1 solution: {}", exit_steps(first_k(1024)).unwrap());
    //Can be optimized with binary search
    let k = (1025..).find_or_first(|i| exit_steps(first_k(*i)).is_none());
    println!("Part 2 solution: {:?}", nums[k.unwrap()-1]);
}
fn exit_steps(blocks: HashSet<&(i64, i64)>) -> Option<i64> {
    let (w,h) = (70, 70);
    let mut pq = PriorityQueue::new();
    let mut visited = HashSet::new();
    pq.push((0, 0), 0);
    while let Some((loc@(x, y), score)) = pq.pop() {
        if blocks.contains(&loc) { continue; }
        if x < 0 || x > w || y < 0 || y > h { continue; }
        if !visited.insert(loc) { continue; }
        if x == w && y == h {
            return Some(-score);
        }
        pq.push((x+1,y), score-1);
        pq.push((x-1,y), score-1);
        pq.push((x,y+1), score-1);
        pq.push((x,y-1), score-1);
    }
    return None;
}