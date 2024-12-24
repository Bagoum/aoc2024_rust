use itertools::Itertools;
use priority_queue::PriorityQueue;
use regex::Regex;
use crate::aoc::ToI64;

struct State {
    ra: i64,
    rb: i64,
    rc: i64
}

pub fn solve(data: String) {
    let mut nums = Regex::new(r"\d+").unwrap().find_iter(data.as_str()).map(|x|x.toi64()).collect_vec();
    let mut state = State { ra: nums.remove(0), rb: nums.remove(0), rc: nums.remove(0) };
    let result = run(&nums, &mut state);
    println!("Part 1 solution: {}", result.into_iter().map(|x|x.to_string()).join(","));

    let mut pq = PriorityQueue::new();
    pq.push(0, 0);
    //One result value is constructed for each 3 bits of the original `ra`, so we construct
    // `ra` 3 bits at a time by iterating over the desired result values in reverse.
    while let Some((nxt, len)) = pq.pop() {
        if len >= nums.len() {
            println!("Part 2 solution: {}", nxt);
            break;
        }
        ((nxt << 3)..((nxt << 3) + 8i64)).filter(|ra|
            run(&nums, &mut State { ra: *ra, rb: 0, rc: 0 })[0] == nums[nums.len()-1-len])
           .for_each(|ra|{ pq.push(ra, len+1); });
    }
}
fn run(steps: &Vec<i64>, state: &mut State) -> Vec<i64> {
    let mut output = vec![];
    let mut idx = 0;
    while idx < steps.len() {
        let (lit, combo) = (steps[idx+1], match steps[idx+1] {
            4 => state.ra,
            5 => state.rb,
            6 => state.rc,
            x => x
        });
        match steps[idx] {
            0 => state.ra = state.ra >> combo,
            1 => state.rb = state.rb ^ lit,
            2 => state.rb = combo % 8,
            3 => if state.ra != 0 { idx = lit as usize; continue; },
            4 => state.rb = state.rb ^ state.rc,
            5 => output.push(combo % 8),
            6 => state.rb = state.ra >> combo,
            _ => state.rc = state.ra >> combo,
        }
        idx += 2;
    }
    output
}