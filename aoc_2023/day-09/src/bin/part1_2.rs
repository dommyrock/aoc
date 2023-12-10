macro_rules! sequence_differences {
    ($vec:expr) => {{
        let mut differences = Vec::new();
        if $vec.len() > 1 {
            for i in 1..$vec.len() {
                differences.push($vec[i] - $vec[i - 1]); //we skip 0 so this is fine
            }
        }
        differences
    }};
}
fn main() {
    let mut sum: i32 = 0;
    include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .into_iter()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .into_iter()
        .for_each(|vec: Vec<i32>| {
            //part1
            let nxt = *vec.last().unwrap() + calc_right(vec);
            //part2
            // let nxt = *vec.first().unwrap() - calc_left(vec);
            sum += nxt;
        });

    println!("result {sum}");
}
fn calc_right(mut v: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    while v.windows(2).any(|w| w[1] - w[0] != 0) {//222 || 000 
        v = sequence_differences!(v);
        let last = v.iter().last().unwrap();
        result.push(*last);
    }
    result.into_iter().sum()
}
fn calc_left(mut v: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    while v.windows(2).any(|w| w[1] - w[0] != 0) {
        v = sequence_differences!(v);
        let num = v[0];
        result.push(num);
    }
    result.iter().rev().fold(0, |acc, x| x - acc)
}
