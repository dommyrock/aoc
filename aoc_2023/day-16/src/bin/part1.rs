use std::collections::HashSet;
use std::collections::VecDeque;
fn main() {
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut q: VecDeque<(isize, isize, isize, isize)> = VecDeque::new();
    q.push_back((0, -1, 0, 1));
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

    println!("{}", coords.len());
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

///Invoke
/// ```
///if_else_version(ch, dc, dr, &mut seen, r, c, &mut q);`
/// ```
fn _if_else_version(
    ch: char,
    mut dc: isize,
    mut dr: isize,
    seen: &mut HashSet<(isize, isize, isize, isize)>,
    r: isize,
    c: isize,
    q: &mut VecDeque<(isize, isize, isize, isize)>,
) {
    if ch == '.' || (ch == '-' && dc != 0) || (ch == '|' && dr != 0) {
        if !seen.contains(&(r, c, dr, dc)) {
            seen.insert((r, c, dr, dc));
            q.push_back((r, c, dr, dc));
        }
    } else if ch == '/' {
        std::mem::swap(&mut dr, &mut dc);
        dr = -dr;
        dc = -dc;
        if !seen.contains(&(r, c, dr, dc)) {
            seen.insert((r, c, dr, dc));
            q.push_back((r, c, dr, dc));
        }
    } else if ch == '\\' {
        std::mem::swap(&mut dr, &mut dc);
        if !seen.contains(&(r, c, dr, dc)) {
            seen.insert((r, c, dr, dc));
            q.push_back((r, c, dr, dc));
        }
    } else {
        let directions = if ch == '|' {
            vec![(1, 0), (-1, 0)]
        } else {
            vec![(0, 1), (0, -1)]
        };
        for (dr, dc) in directions {
            if !seen.contains(&(r, c, dr, dc)) {
                seen.insert((r, c, dr, dc));
                q.push_back((r, c, dr, dc));
            }
        }
    }
}
