fn main() {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut sum: usize = 0;

    include_str!("input.txt").lines().for_each(|ln| {
        matrix.push(ln.chars().collect());
    });
    rotate_left_90(&mut matrix);

    matrix.iter().for_each(|ln| {
      //groups of round rocks split by "#" flat rocks
        let round_rock_groups: Vec<usize> = ln
            .iter()
            .collect::<String>()
            .split('#')
            .map(|s| s.chars().filter(|c| *c == 'O').count())
            .collect();
        let flat_indices: Vec<usize> = ln
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == '#' { Some(i) } else { None })
            .collect();
        let mut from: Vec<usize> = vec![ln.len()]; //10
        from.extend(&flat_indices);

        //sum each group
        for (idx, &group_count) in round_rock_groups.iter().enumerate() {
            if group_count > 0 {
                let mut f = from[idx];
                let mut to = from[idx] - group_count;

                //handle everything after 1st '#'
                if idx > 0 {
                    f = ln.len() - from[idx];
                    to = f - group_count;
                    //rev because DESC.sum() doesnt work for range
                    let row_sum: usize = (to..f).into_iter().rev().sum();

                    sum += row_sum;
                    continue;
                }

                let row_sum: usize = (to + 1..=f).into_iter().sum();
                sum += row_sum
            }
        }
    });
    println!("Sum : {sum}");
}

/// Transpose the matrix
fn rotate_left_90(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();
    for i in 0..n {
        for j in i + 1..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }
}
