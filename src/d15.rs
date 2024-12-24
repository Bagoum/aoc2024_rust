use itertools::Itertools;

pub fn solve(data: String) {
    let data = data.split("\r\n\r\n").collect_vec();
    let (mut rx, mut ry) = (0, 0);
    let mut matrix = data[0].lines().enumerate().map(|(y,l)|
        l.chars().enumerate().map(|(x,c)| {
            if c == '@' { (rx, ry) = (x as i64, y as i64); }
            c
        }).collect_vec()).collect_vec();
    let (mut rwx, mut rwy) = (rx*2, ry);
    let mut widematrix = matrix.iter().map(|l| l.iter().flat_map(|c| match c {
        'O' => vec!['[', ']'],
        '@' => vec!['@', '.'],
        x => vec![*x, *x]
    }).collect_vec()).collect_vec();
    data[1].chars().filter_map(|c| match c {
        '<' => Some((-1, 0)),
        '>' => Some((1, 0)),
        '^' => Some((0, -1)),
        'v' => Some((0, 1)),
        _ => None
    }).for_each(|(dx, dy)| {
        if trypush((rx, ry), (dx, dy), &matrix, false) {
            dopush((rx, ry), (dx, dy), &mut matrix, false);
            (rx, ry) = (rx + dx, ry + dy);
        }
        if trypush((rwx, rwy), (dx, dy), &widematrix, false) {
            dopush((rwx, rwy), (dx, dy), &mut widematrix, false);
            (rwx, rwy) = (rwx + dx, rwy + dy);
        }
    });
    let score = |map: &Vec<Vec<_>>, ch|->usize { map.iter().enumerate().map(|(y,l)|
        l.iter().enumerate().filter_map(|(x,c)|(*c==ch).then_some(x+100*y))
            .sum::<usize>()).sum() };
    println!("Part 1 solution: {}", score(&matrix, 'O'));
    println!("Part 2 solution: {}", score(&widematrix, '['));

}
fn dopush((x,y): (i64, i64), (dx,dy): (i64, i64), map: &mut Vec<Vec<char>>, is_dep: bool) {
    match map[y as usize][x as usize] {
        c @ ('@'|'O'|'['|']') => {
            dopush((x+dx, y+dy), (dx, dy), map, false);
            map[(y+dy) as usize][(x+dx) as usize] = map[y as usize][x as usize];
            map[y as usize][x as usize] = '.';
            if dy != 0 && !is_dep && (c == '[' || c == ']') {
                dopush((x+if c == '[' {1} else {-1}, y), (dx, dy), map, true);
            }
        }
        _ => {}
    }
}
fn trypush((x,y): (i64, i64), (dx,dy): (i64, i64), map: &Vec<Vec<char>>, is_dep: bool) -> bool {
    match map[y as usize][x as usize] {
        '#' => false,
        '.' => true,
        '[' => trypush((x+dx, y+dy), (dx, dy), map, false)
            && (is_dep || dy == 0 || trypush((x+1, y), (dx, dy), map, true)),
        ']' => trypush((x+dx, y+dy), (dx, dy), map, false)
            && (is_dep || dy == 0 || trypush((x-1, y), (dx, dy), map, true)),
        _ => trypush((x+dx, y+dy), (dx, dy), map, false)
    }
}