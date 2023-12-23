use std::collections::HashSet;
use std::collections::VecDeque;

//maximize the energized tiles

fn main() {
    let mut grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut max_val = 0;
    //Start just of the grid (because we check calc() after we mooved)
    //Rows left | Right
    for r in 0..grid.len() {
        let row_len = grid[0].len() as isize;
        max_val = std::cmp::max(max_val, calc(&mut grid, r as isize, -1, 0, 1));
        max_val = std::cmp::max(max_val, calc(&mut grid, r as isize, row_len, 0, 1));
    }
    //Columns Top | Bottom
    for c in 0..grid.len() {
        let len = grid.len() as isize;
        max_val = std::cmp::max(max_val, calc(&mut grid, -1, c as isize, 1, 0));
        max_val = std::cmp::max(max_val, calc(&mut grid, len, c as isize, -1, 0));
    }

    println!("{}", max_val);
}

fn calc(grid: &mut Vec<Vec<char>>, r: isize, c: isize, dr: isize, dc: isize) -> usize {
    let mut q: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();
    q.push_back((r, c, dr, dc));
    let mut seen: HashSet<(isize, isize, isize, isize)> = HashSet::new();

    while let Some((mut r, mut c, mut dr, mut dc)) = q.pop_front() {
        r = r + dr;
        c += dc;

        if r < 0 || r >= grid.len() as isize || c < 0 || c >= grid[0].len() as isize {
            continue;
        }

        let ch = grid[r as usize][c as usize];

        match ch {
            '.' | '-' if dc != 0 => {
                not_seen_than_insert(&mut seen, r, c, dr, dc, &mut q);
            }
            '.' | '|' if dr != 0 => {
                not_seen_than_insert(&mut seen, r, c, dr, dc, &mut q);
            }
            '/' => {
                std::mem::swap(&mut dr, &mut dc);
                dr = -dr;
                dc = -dc;
                if !seen.contains(&(r, c, dr, dc)) {
                    seen.insert((r, c, dr, dc));
                    q.push_back((r, c, dr, dc));
                }
            }
            '\\' => {
                std::mem::swap(&mut dr, &mut dc);
                if !seen.contains(&(r, c, dr, dc)) {
                    seen.insert((r, c, dr, dc));
                    q.push_back((r, c, dr, dc));
                }
            }
            _ => {
                let directions = if ch == '|' {
                    vec![(1, 0), (-1, 0)]
                } else {
                    vec![(0, 1), (0, -1)]
                };
                for (dr, dc) in directions {
                    if seen.insert((r, c, dr, dc)) {
                        q.push_back((r, c, dr, dc));
                    }
                }
            }
        }
    }
    let coords: HashSet<(isize, isize)> = seen.iter().map(|(r, c, _, _)| (*r, *c)).collect();

    coords.len()
}

fn not_seen_than_insert(
    seen: &mut HashSet<(isize, isize, isize, isize)>,
    r: isize,
    c: isize,
    dr: isize,
    dc: isize,
    q: &mut VecDeque<(isize, isize, isize, isize)>,
) {
    if !seen.contains(&(r, c, dr, dc)) {
        seen.insert((r, c, dr, dc));
        q.push_back((r, c, dr, dc));
    }
}
