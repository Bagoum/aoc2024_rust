use std::collections::hash_map::Entry;
use std::collections::HashMap;
use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let main_keypad = HashMap::from([
        ('7', (0,0)),
        ('8', (1,0)),
        ('9', (2,0)),
        ('4', (0,1)),
        ('5', (1,1)),
        ('6', (2,1)),
        ('1', (0,2)),
        ('2', (1,2)),
        ('3', (2,2)),
        ('0', (1,3)),
        ('A', (2,3)),
    ]);
    let mut total1 = 0;
    let mut total2 = 0;
    for seq in data.lines() {
        let targets = seq.chars().map(|c| *main_keypad.get(&c).unwrap()).collect_vec();
        let mut tstart = (2,3);
        let mut targets_dct = HashMap::new();
        for t in targets.iter() {
            match targets_dct.entry((tstart, *t)) {
                Entry::Occupied(v) => *v.into_mut() += 1,
                Entry::Vacant(v) => { v.insert(1); },
            }
            tstart = *t;
        }
        let mut raised = raise_seq_level(targets_dct, (0, 3));
        for ii in 0..25 {
            if ii == 2 {
                total1 += raised.values().sum::<i64>() * seq[..3].toi64();
            }
            raised = raise_seq_level(raised, (0, 0));
        }
        total2 += raised.values().sum::<i64>() * seq[..3].toi64();
    }
    println!("Part 1 solution: {total1}");
    println!("Part 2 solution: {total2}");
}

fn raise_seq_level(steps: HashMap<((i32, i32), (i32, i32)), i64>, empty: (i32, i32)) -> HashMap<((i32, i32), (i32, i32)), i64> {
    let mut out = HashMap::new();
    for ((prev, nxt), ct) in steps {
        let mut raised_start = (2, 0);
        for raised_nxt in raise_level(prev, nxt, empty) {
            match out.entry((raised_start, raised_nxt)) {
                Entry::Occupied(v) => *v.into_mut() += ct,
                Entry::Vacant(v) => { v.insert(ct); },
            };
            raised_start = raised_nxt;
        }
    }
    out
}
fn raise_level((px,py): (i32, i32), (nx,ny): (i32, i32), empty: (i32, i32)) -> Vec<(i32, i32)> {
    let (dx, dy) = (nx - px, ny - py);
    let move_x = (0..dx.abs()).map(|_| if dx > 0 { (2,1) } else { (0,1) }).collect_vec();
    let move_y = (0..dy.abs()).map(|_| if dy > 0 { (1,1) } else { (1,0) }).collect_vec();
    let x_first = dx != 0 && !(py == empty.1 && nx == empty.0);
    let y_first = dy != 0 && !(px == empty.0 && ny == empty.1);
    //<^A is better than ^<A
    if x_first && (!y_first || dx < 0) {
        vec![&move_x[..], &move_y[..], &vec![(2,0)]].concat()
    } else if y_first {
        vec![&move_y[..], &move_x[..], &vec![(2,0)]].concat()
    } else {
        vec![(2,0)]
    }
}