use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(data: String) {
    let mut height_map = HashMap::new();
    for (row, l) in data.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            height_map.entry(c as i32 - '0' as i32).or_insert(HashSet::new()).insert((col as i64, row as i64));
        }
    }
    let reachable_nines = height_map.iter().flat_map(|(h, xs)| xs.iter().map(|x|
            (x, RefCell::new(if h == &9 { vec![x] } else { vec![] } )))
        .collect_vec()).collect::<HashMap<_,_>>();
    let deltas = [(0i64,1i64),(1,0),(0,-1),(-1,0)];
    for height in (1..=9).rev() {
        for (x,y) in height_map[&height].iter() {
            let reachable = reachable_nines[&(*x,*y)].borrow();
            for (dx, dy) in deltas.iter() {
                if height_map[&(height-1)].contains(&(x+dx,y+dy)) {
                    //refcell allows borrow_mut here while another key is borrowed 3 lines above
                    reachable_nines[&(x+dx,y+dy)].borrow_mut().extend(reachable.iter())
                }
            }
        }
    }

    println!("Part 1 solution: {}", height_map[&0].iter().map(|x|
        reachable_nines[x].borrow().iter().unique().count()).sum::<usize>());
    println!("Part 2 solution: {}", height_map[&0].iter().map(|x|
        reachable_nines[x].borrow().len()).sum::<usize>());
}

