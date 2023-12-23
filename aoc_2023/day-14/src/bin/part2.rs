use std::collections::HashSet;

fn main() {
    let mut grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .collect();

    // for l in &grid {
    //     println!("{:?}", l);
    // }
    cycle(&mut grid);

    let mut seen: HashSet<Vec<Vec<char>>> = std::iter::once(grid.clone()).collect();
    let mut buffer: Vec<Vec<Vec<char>>> = vec![grid.clone()];
    let mut iter = 0;

    loop {
        iter += 1;
        cycle(&mut grid);
        if seen.contains(&grid) {
            break;
        }
        seen.insert(grid.clone());
        buffer.push(grid.clone());
    }
    let first = buffer.iter().position(|x| *x == grid).unwrap();

    // print!("(1B -  {}) % ({iter} - {first}) + {first}\n", first - 1);
    grid = buffer[(1_000_000_000 - first - 1) % (iter - first) + first].clone();

    let total = grid
        .iter()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&&c| c == 'O').count() * (grid.len() - idx))
        .sum::<usize>();
    println!("\nTotal : {}", total);
}

/// Rotates the grid 90 degrees counterclockwise and then flipping it horizontally.</br>
/// x4 = 1 cycle
fn cycle(grid: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        // Transpose the grid
        let mut transposed = vec![vec![' '; grid.len()]; grid[0].len()];
        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                transposed[j][i] = cell;
            }
        }
        *grid = transposed;

        // Sort the blocks of 'O' characters in each row in descending order
        for row in grid.iter_mut() {
            let mut blocks: Vec<String> = row
                .iter()
                .collect::<String>()
                .split('#')
                .map(String::from)
                .collect();

            for block in &mut blocks {
                let mut chars: Vec<char> = block.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                *block = chars.into_iter().collect();
            }
            *row = blocks.join("#").chars().collect();
        }

        // Reverse the order of characters in each row
        for row in grid.iter_mut() {
            row.reverse();
        }
        /* flips the grid against vertical axis
        1 2 3
        4 5 6
        7 8 9
        ------into
        3 2 1
        6 5 4
        9 8 7
         */
    }
}
//python Version
// https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day14p2.py
