use std::time::Instant;

macro_rules! sequence_differences {
    ($vec:expr) => {{
        $vec.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>()
    }};
}

fn main() {
    let start = Instant::now();
    let sum = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .fold(0, |acc, vec| 
            //part1
            acc + *vec.last().unwrap() + calc_right(vec)
            //part2
            // acc + *vec.first().unwrap() - calc_left(vec) 
        );

    println!("result {sum}");
    println!("Elapsed : {:?}", start.elapsed());
}

fn calc_right(mut v: Vec<i32>) -> i32 {
    let mut result = 0;
    while v.windows(2).any(|w| w[1] - w[0] != 0) {
        v = sequence_differences!(v);
        result += v.last().unwrap();
    }
    result
}
fn calc_left(mut v: Vec<i32>) -> i32 {
    let mut result: Vec<i32> = Vec::new();
    while v.windows(2).any(|w| w[1] - w[0] != 0) {
        v = sequence_differences!(v);
        result.push(v[0]);
    }
    result.iter().rev().fold(0, |acc, x| x - acc)
}
