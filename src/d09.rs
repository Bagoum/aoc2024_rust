use itertools::Itertools;

pub fn solve(data: &str) {
    let data = data.chars().map(|c| c as i64 -'0' as i64).collect_vec();
    let mut idxs = data.iter().scan(0, |acc, x| { *acc += x; Some(*acc) }).collect_vec();
    idxs.insert(0, 0);
    //sum x,x+1...x+k-1 = (2x+k-1)*k/2
    let hash = |idx, bidx:usize, len| (bidx as i64/2) * ((2*idx+len-1) * len) / 2;
    let mut data1 = data.clone();
    let mut back = data.len()-1;
    let mut checksum = 0;
    let mut front = 0;
    while front <= back {
        checksum += hash(idxs[front], front, data1[front]);
        if front == back { break; }
        let emptylen = data1[front + 1];
        let mut filled = 0;
        while emptylen > filled {
            let consume = (emptylen-filled).min(data1[back]);
            checksum += hash(idxs[front+1] + filled, back, consume);
            filled += consume;
            data1[back] -= consume;
            if data1[back] == 0 {
                back -= 2;
            }
        }
        front += 2;
    }
    println!("Part 1 solution: {checksum}");

    let mut checksum = 0;
    let mut consumed = (0..data.len()).map(|_| 0).collect_vec();
    for back in (0..data.len()).step_by(2).rev() {
        let bblocklen = data[back];
        let mut found = false;
        if back > 0 {
            for front in (1..back).step_by(2) {
                if bblocklen <= data[front] - consumed[front] {
                    found = true;
                    checksum += hash(idxs[front] + consumed[front], back, bblocklen);
                    consumed[front] += bblocklen;
                    break;
                }
            }
        }
        if !found {
            checksum += hash(idxs[back], back, bblocklen);
        }
    }
    println!("Part 2 solution: {checksum}");

}

