use itertools::Itertools;

pub fn solve(data: String) {
    let matrix = data.lines().map(|l| l.chars().map(|c| c as i64).collect_vec()).collect_vec();
    let height = matrix.len() as i64;
    let width = matrix[0].len() as i64;
    let mut fill = (0..height).map(|_| (0..width).map(|_| 0).collect_vec()).collect_vec();
    let mut groups = 1;
    for y in 0..height {
        for x in 0..width {
            if fill[y as usize][x as usize] == 0 {
                floodfill(&matrix, &mut fill, (x, y), groups);
                groups += 1;
            }
        }
    }
    let as_group = |(x,y), grp| y>=0 && y<height && x>=0 && x<width && fill[y as usize][x as usize] == grp;
    let mut area = (0..groups).map(|_|0i64).collect_vec();
    let (mut perimeter, mut corners) = (area.clone(), area.clone());
    let ds = [(0i64, 1i64),(1, 0),(0, -1),(-1, 0),(0, 1)];
    for y in 0..height {
        for x in 0..width {
            let grp = fill[y as usize][x as usize];
            area[grp] += 1;
            for di in 0..4 {
                if as_group((x + &ds[di].0, y + &ds[di].1), grp) {
                    //Inner corner: a diagonal is a different group but the two adjacents are the same
                    if as_group((x + &ds[di+1].0, y + &ds[di+1].1), grp) &&
                        !as_group((x + &ds[di].0 + &ds[di+1].0, y + &ds[di].1 + &ds[di+1].1), grp) {
                        corners[grp] += 1;
                    }
                } else {
                    perimeter[grp] += 1;
                    //Outer corner: two adjacents are a different group
                    if !as_group((x + &ds[di+1].0, y + &ds[di+1].1), grp) {
                        corners[grp] += 1;
                    }
                }
            }
        }
    }
    println!("Part 1 solution: {}", area.iter().zip(perimeter).map(|(x,y)| x*y).sum::<i64>());
    println!("Part 2 solution: {}", area.iter().zip(corners).map(|(x,y)| x*y).sum::<i64>());
}

fn floodfill(matrix: &Vec<Vec<i64>>, fill: &mut Vec<Vec<usize>>, (x,y): (i64, i64), id: usize) {
    fill[y as usize][x as usize] = id;
    for (dx,dy) in [(0i64,1i64),(1,0),(0,-1),(-1,0)].into_iter() {
        let (nx,ny) = (x+dx, y+dy);
        if ny >= 0 && ny < matrix.len() as i64 && nx >= 0 && nx < matrix[0].len() as i64 &&
            matrix[ny as usize][nx as usize] == matrix[y as usize][x as usize] && fill[ny as usize][nx as usize] == 0 {
            floodfill(matrix, fill, (nx, ny), id);
        }
    }
}
