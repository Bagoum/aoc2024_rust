use itertools::Itertools;
use regex::Regex;
use crate::aoc::ToI64;

pub fn solve(data: String) {
    let r = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let w = 101;
    let h = 103;
    let problems = data.lines().map(|l| {
        let m = r.captures(l).unwrap();
        return (m.get(1).unwrap().toi64(), m.get(2).unwrap().toi64(),
                m.get(3).unwrap().toi64(),m.get(4).unwrap().toi64());
    }).collect_vec();
    //Look for the timestep which minimizes the sum of variances
    //Can be optimized by separately calculating variance of X over T%width and Y over T%height
    let mut positions = problems.iter().map(|(x,y,_,_)| (*x,*y)).collect_vec();
    let mut variances = vec![f64::MAX];
    for t in 1..=(w * h) {
        let mut tx = 0;
        let mut ty = 0;
        for i in 0..positions.len() {
            let (ref mut x, ref mut y) = positions[i];
            (*x,*y) = (((*x+problems[i].2) % w + w) % w, ((*y+problems[i].3) % h + h) % h);
            tx += *x;
            ty += *y;
        }
        let ax: f64 = tx as f64 / positions.len() as f64;
        let ay: f64 = ty as f64 / positions.len() as f64;
        variances.push(positions.iter().map(|(x,y)| (*x as f64-ax).powi(2)+(*y as f64-ay).powi(2)).sum::<f64>());
        if t == 100 {
            let quadrants = positions.iter().map(|(x,y)| (x-w/2, y-h/2)).filter_map(|(x,y)|
                if x == 0 || y == 0 { None } else { Some(if x < 0 {1} else {0} + if y < 0 {2} else {0}) }
            ).counts();
            println!("Part 1 solution: {}", quadrants.values().product::<usize>());
        }
    }
    println!("Part 2 solution: {}", variances.into_iter().enumerate().min_by(|a,b| a.1.total_cmp(&b.1)).unwrap().0);
}

