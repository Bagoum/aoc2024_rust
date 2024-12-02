use itertools::Itertools;

pub fn solve(data: String) {
    let (mut left, mut right): (Vec<_>, Vec<_>) =
        data.lines().map(|x| {
            let split = x.split_whitespace().collect_vec();
            (split[0].parse::<i64>().unwrap(), split[1].parse::<i64>().unwrap())
        }).unzip();
    left.sort_unstable();
    right.sort_unstable();

    let sum: i64 = left.iter().zip(right.iter()).map(|(l, r)| (l-r).abs()).sum();
    println!("Part 1 result: {sum}");

    let counts_map = right.iter().counts();
    let similarity: i64 = left.iter()
        .map(|x| i64::try_from(*counts_map.get(x).unwrap_or(&0)).unwrap() * x)
        .sum();
    println!("Part 2 result: {similarity}");
}