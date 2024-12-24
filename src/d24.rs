use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

pub fn solve(data: String) {
    let data = data.split("\r\n\r\n").collect_vec();
    let r1 = Regex::new(r"(\w+): (\d)").unwrap();
    let r2 = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    let initials = r1.captures_iter(data[0]).map(|m| (m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str() == "1")).collect::<HashMap<_,_>>();
    let ops = r2.captures_iter(data[1]).map(|m| {
        let (a, b) = (m.get(1).unwrap().as_str(), m.get(3).unwrap().as_str());
        (m.get(4).unwrap().as_str(), (a.min(b), m.get(2).unwrap().as_str(), a.max(b)))
    }
    ).collect::<HashMap<_,_>>();
    let mut is_input_to = HashMap::new();
    for (res, (a, _, b)) in ops.iter() {
        is_input_to.entry(*a).or_insert(vec![]).push(*res);
        is_input_to.entry(*b).or_insert(vec![]).push(*res);
    }
    let mut resolve = initials.clone();
    let mut total = 0i64;
    for wire in ops.keys().filter(|k| k.starts_with("z")).sorted().rev() {
        total = total * 2 + if solve_wire(wire, &ops, &mut resolve) { 1 } else { 0 };
    }
    println!("Part 1 solution: {total}");
    let zfinal = format!("z{:0>2}", initials.len()/2);
    let errors = ops.iter().filter(|(res, (a, op, _))| {
        /*  xn & yn = fn (first-carry) (f0 => c0)
            xn ^ yn = wn (digit-w/o-carry) (w0 => z0)
            wn ^ cn- = zn (digit)
            wn & cn- = sn (second-carry)
            fn | sn = cn (carry) */
        let mut expect_usage = if **res == zfinal { vec![] } else { match *op {
            "AND" if *a != "x00" => vec!["OR"], //fn or sn
            "XOR" => if a.starts_with('x') && *a != "x00" { vec!["AND", "XOR"] } else { vec![] }, //wn or zn
            _ => vec!["AND", "XOR"] //cn
        }};
        match is_input_to.get(*res) {
            None => expect_usage.len() > 0,
            Some(set) => !vec_eq(&mut set.iter().map(|nxt| ops.get(nxt).unwrap().1).collect_vec(), &mut expect_usage)
        }
    }).map(|(k,_)| k).sorted().join(",");
    println!("Part 2 solution: {}", errors);
}
fn solve_wire<'a>(wire: &'a str, ops: &HashMap<&str, (&'a str,&str,&'a str)>, resolve: &mut HashMap<&'a str, bool>) -> bool {
    match resolve.get(wire) {
        Some(v) => *v,
        None => {
            let (a, op, b) = ops.get(wire).unwrap();
            let a = solve_wire(a, ops, resolve);
            let b = solve_wire(b, ops, resolve);
            let result = match *op {
                "AND" => a && b,
                "OR" => a || b,
                _ => a != b,
            };
            resolve.insert(wire, result);
            result
        }
    }
}
fn vec_eq<T>(a: &mut Vec<T>, b: &mut Vec<T>)-> bool where T: Eq + Ord {
    if a.len() != b.len() { return false; }
    a.sort();
    b.sort();
    for ii in 0..a.len() {
        if a[ii] != b[ii] { return false; }
    }
    return true;
}