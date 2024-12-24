use itertools::Itertools;
use regex::Regex;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let r = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    let problems = data.split("\r\n\r\n").map(|p| {
        let xys = r.captures_iter(p).collect_vec();
        return (xys[0].get(1).unwrap().toi64(), xys[0].get(2).unwrap().toi64(),
                xys[1].get(1).unwrap().toi64(), xys[1].get(2).unwrap().toi64(),
                xys[2].get(1).unwrap().toi64(), xys[2].get(2).unwrap().toi64()
        );
    }).collect_vec();

    let total_cost = |add_extra|->i64 { problems.iter().filter_map(|(ax,ay,bx,by,tx,ty)| {
        // [tx]  =  [ ax  bx ] [ ai ]
        // [ty]     [ ay  by ] [ bi ]
        let tx = tx + if add_extra { 10000000000000 } else { 0 };
        let ty = ty + if add_extra { 10000000000000 } else { 0 };
        let det = ax * by - ay * bx;
        let ai = (by * tx - bx * ty) / det;
        let bi = (-ay * tx + ax * ty) / det;
        (ax*ai+bx*bi==tx && ay*ai+by*bi==ty).then_some(3*ai + bi)
    }).sum() };

    println!("Part 1 solution: {}", total_cost(false));
    println!("Part 2 solution: {}", total_cost(true));
}

