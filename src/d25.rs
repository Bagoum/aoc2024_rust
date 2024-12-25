use itertools::Itertools;

pub fn solve(data: String) {
    let data = data.split("\r\n\r\n").collect_vec();
    let mut locks = vec![];
    let mut keys = vec![];
    let check_fit = |x: &Vec<i64>, ys: &Vec<Vec<i64>>|
        ys.iter().filter(|y| x.iter().zip(*y).all(|(a,b)| a + b <= 7)).count();
    let mut matches = 0;
    for obj in data {
        let mut is_lock = false;
        let mut heights = vec![0,0,0,0,0];
        for (i, l) in obj.lines().enumerate() {
            for (j, c) in l.chars().enumerate() {
                if c == '#' {
                    heights[j] += 1;
                    if i == 0 && j == 0 { is_lock = true };
                }
            }
        }
        if is_lock {
            matches += check_fit(&heights, &keys);
            locks.push(heights);
        } else {
            matches += check_fit(&heights, &locks);
            keys.push(heights);
        }
    }
    println!("Part 1 solution: {matches}");
}
