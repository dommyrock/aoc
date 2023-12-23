fn main() {
    let grid: Vec<Vec<char>> = include_str!("input.txt")
        .lines()
        .map(|ln| ln.chars().collect::<Vec<char>>())
        .collect();

    let mut total = 0;
    let height = grid.len();

    for col in 0..grid[0].len() {
        //Transpose grid
        let mut blocks: Vec<String> = grid
            .iter()
            .map(|row| row[col])
            .collect::<String>()
            .split('#')
            .map(String::from)
            .collect();

        let mut index = height + 1;
        for block in blocks.iter_mut() {
            let n = block.matches('O').count();
            if n > 0 {
                total += n * index - n * (n + 1) / 2;
            }
            index -= block.len() + 1;
        }
    }
    println!("Sum: {}", total);
}
//Transpose grid (flipping it over its diagonal)
/*
1 2 3
4 5 6

Becomes
1 4
2 5
3 6

 */
