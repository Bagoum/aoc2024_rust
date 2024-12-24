use std::collections::HashMap;
use itertools::Itertools;
use priority_queue::PriorityQueue;

pub fn solve(data: String) {
    let mut distances = HashMap::new();
    let mut pq = PriorityQueue::new();
    let mut start = (0,0);
    let matrix = data.lines().enumerate().map(|(y,l)|
        l.chars().enumerate().map(|(x,c)| match c {
            'E' => { pq.push((x as i64,y as i64), 0i64); '.' },
            'S' => { start = (x as i64, y as i64); '.' },
            c => c
        }).collect_vec()).collect_vec();
    let deltas = [(0i64,1i64),(1,0),(0,-1),(-1,0)];
    while let Some((nxt@(x,y), dist)) = pq.pop() {
        if distances.contains_key(&nxt) { continue; }
        if matrix[y as usize][x as usize] == '#' { continue; }
        distances.insert(nxt, -dist);
        for (dx, dy) in deltas {
            pq.push((x+dx,y+dy), dist-1);
        }
    }

    let k_step_cheats = |k: i64, lim:i64| distances.iter().map(|((x,y), dist)| {
        let mut total = 0;
        for dx in 0..=k {
            let maxdy = (k - dx).abs();
            for dy in (if dx == 0 { 0 } else {-maxdy})..=maxdy {
                match distances.get(&(x+dx, y+dy)) {
                    None => {}, Some(cdist) =>
                        if (cdist - dist).abs() >= lim + dx + dy.abs()
                        { total += 1 }
                }
            }
        }
        total
    }).sum::<i64>();

    println!("Part 1 solution: {}", k_step_cheats(2, 100));
    println!("Part 2 solution: {}", k_step_cheats(20, 100));
}
