use std::collections::HashSet;

fn main() {
    let mut res = 0;
    include_str!("input.txt")
        .lines()
        .into_iter()
        .for_each(|ln| {
            let vec: Vec<String> = ln
                .chars()
                .skip(8)
                .collect::<String>()
                .split('|')
                .map(|s| s.to_string())
                .collect();
            let (w, me) = (vec[0].clone(), vec[1].clone());

            let winners: HashSet<usize> = get_nums(w).into_iter().collect();
            let guessers: HashSet<usize> = get_nums(me).into_iter().collect();
            let intersection: HashSet<_> = guessers.intersection(&winners).collect();
            if intersection.len() > 0 {
                let sum = intersection.into_iter().fold(0.5, |acc, _| (acc * 2 as f32)) as usize;
                res += sum;
            } 
        });
        println!("{res}");
}

fn get_nums(s: String) -> Vec<usize> {
    s.split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}