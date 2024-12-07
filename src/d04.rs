use itertools::Itertools;

pub fn solve(data: String) {
    let matrix = data.lines().map(|l| l.as_bytes()).collect_vec();
    let deltas = vec![(-1,0),(1,0),(0,-1),(0,1),(-1,-1),(-1,1),(1,-1),(1,1)];
    let found1: i32 = (0..matrix.len()).map(|y| (0..matrix[y].len()).map(|x| deltas.iter().map(|(dx, dy)|
        matcher1(&matrix, x as i32, y as i32, *dx, *dy, "XMAS".as_bytes())).sum::<i32>()).sum::<i32>()).sum();
    println!("Part 1 result: {found1}");

    let found2: i32 = (0..matrix.len()).map(|y| (0..matrix[y].len()).map(|x|
        if cmp(&matrix, x as i32, y as i32, 'A' as u8) == 1 &&
            nearby_in_x(&matrix, x as i32, y as i32, 'S' as u8) == 2 &&
            nearby_in_x(&matrix, x as i32, y as i32, 'M' as u8) == 2 &&
            matrix[y-1][x-1] != matrix[y+1][x+1] { 1 } else { 0 }
    ).sum::<i32>()).sum();
    println!("Part 2 result: {found2}")
}

pub fn cmp(matrix: &Vec<&[u8]>, px: i32, py: i32, against: u8) -> i32 {
    if py < 0 || py >= matrix.len() as i32 || px < 0 || px >= matrix[py as usize].len() as i32
        || matrix[py as usize][px as usize] != against { 0 } else { 1 }
}

pub fn matcher1(matrix: &Vec<&[u8]>, px: i32, py: i32, dx: i32, dy: i32, against: &[u8]) -> i32 {
    if against.is_empty() {
        return 1;
    } else if cmp(matrix, px, py, against[0]) == 0 {
        return 0;
    }
    matcher1(matrix, px+dx, py+dy, dx, dy, &against[1..])
}

pub fn nearby_in_x(matrix: &Vec<&[u8]>, px: i32, py: i32, against: u8) -> i32 {
    cmp(matrix, px-1,py-1, against) + cmp(matrix, px-1,py+1, against) +
        cmp(matrix, px+1,py-1, against) + cmp(matrix, px+1,py+1, against)
}

