use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

pub fn solve(data: String) {
    let data = data.split("\r\n\r\n").collect_vec();
    let r1 = Regex::new(r"(\w+): (\d)").unwrap();
    let r2 = Regex::new(r"(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
    let initials = r1.captures_iter(data[0]).map(|m|
        (m.get(1).unwrap().as_str(), m.get(2).unwrap().as_str() == "1")).collect::<HashMap<_,_>>();
    let ops = r2.captures_iter(data[1]).map(|m| {
        let (a, b) = (m.get(1).unwrap().as_str(), m.get(3).unwrap().as_str());
        (m.get(4).unwrap().as_str(), (a.min(b), m.get(2).unwrap().as_str(), a.max(b)))
    }).collect::<HashMap<_,_>>();
    let mut usages = HashMap::new();
    for (a, _, b) in ops.values() {
        *usages.entry(*a).or_insert(0) += 1;
        *usages.entry(*b).or_insert(0) += 1;
    }
    let total = ops.keys().filter(|k| k.starts_with("z")).sorted().rev()
        .fold(0i64, |acc, x| acc * 2 + if solve_wire(x, &ops, &initials) { 1 } else { 0 });
    println!("Part 1 solution: {total}");

    let zfinal = format!("z{:0>2}", initials.len()/2);
    //NB: this does not capture all *possible* errors. It only captures
    // errors where wires with different usage requirements are swapped.
    // eg. If f12 was swapped with s14, this wouldn't capture it.
    //Running this does end up capturing all 8 errors on my input.
    let errors = ops.iter().filter(|(res, (a, op, _))|
        /*  xn & yn = fn (first-carry)
            xn ^ yn = wn (digit-w/o-carry)
            wn ^ cn- = zn (digit)
            wn & cn- = sn (second-carry)
            fn | sn = cn (carry) */
        match *op {
            //fn/sn: 1 usage (x0 & y0 = c0; no f0)
            "AND" if *a != "x00" => 1,
            //wn: 2 usages; zn: 0 usages (x0 ^ y0 = z0; no w0)
            "XOR" => if a.starts_with('x') && *a != "x00" { 2 } else { 0 },
            //cn: 2 usages (f44 | s44 = z45; no c44)
            _ => if **res != zfinal { 2 } else { 0 }
        } != *usages.get(*res).unwrap_or(&0)
    ).map(|(k,_)| k).sorted().join(",");
    println!("Part 2 solution: {}", errors);
}

fn solve_wire<'a>(wire: &'a str, ops: &HashMap<&str, (&'a str,&str,&'a str)>, resolve: &HashMap<&'a str, bool>) -> bool {
    if let Some(r) = resolve.get(wire) { return *r; }
    let (a, op, b) = ops.get(wire).unwrap();
    let a = solve_wire(a, ops, resolve);
    let b = solve_wire(b, ops, resolve);
    match *op {
        "AND" => a && b,
        "OR" => a || b,
        _ => a != b,
    }
}