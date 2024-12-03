use itertools::Itertools;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let reports = data.lines().map(|l| l.split_whitespace().map(|x| x.toi64()).collect_vec()).collect_vec();
    let safe_count = |dampen:bool| reports.iter().filter(|data| {
        let deltas = data.windows(2).map(|w| w[1]-w[0]).collect_vec();
        return all_satisfy(&deltas, &(|x| 0 > x && x >= -3), dampen) ||
            all_satisfy(&deltas, &(|x| 0 < x && x <= 3), dampen);
    }).count();

    println!("Part 1 result: {}", safe_count(false));
    println!("Part 2 result: {}", safe_count(true));
}

fn all_satisfy<F>(deltas: &[i64], f: &F, dampen: bool) -> bool where F:Fn(i64)->bool {
    for ii in 0..deltas.len() {
        if !f(deltas[ii]) {
            if !dampen {
                return false;
            }
            return
                //merge with previous or delete if first element, then check the rest of the data
                ((ii == 0 || f(deltas[ii-1] + deltas[ii]))
                        && all_satisfy(&deltas[ii+1..], f, false)) ||
                //if last element, then just delete and we are OK
                ii == deltas.len()-1 ||
                //merge with next, then check the rest of the data
                (f(deltas[ii] + deltas[ii+1]) && all_satisfy(&deltas[ii+2..], f, false));
        }
    }
    true
}