use std::collections::HashMap;

fn count(num: i64, blinks: usize, cache: &mut HashMap<(i64, usize), i64>) -> i64 {
    if let Some(&result) = cache.get(&(num, blinks)) {
        return result;
    }

    let result = match (blinks, num) {
        (0, _) => 1,
        (_, 0) => count(1, blinks - 1, cache),
        _ => {
            let digits = ((num as f64).log10().floor() + 1.0) as usize;
            if digits % 2 != 0 {
                count(num * 2024, blinks - 1, cache)
            } else {
                let half_l = digits / 2;
                //count left num / count right num
                count(num / 10_i64.pow(half_l as u32), blinks - 1, cache)
                    + count(num % 10_i64.pow(half_l as u32), blinks - 1, cache)
            }
        }
    };
    cache.insert((num, blinks), result);
    result
}

fn main() {
    let data = include_str!("./input1.txt")
        .lines()
        .flat_map(|ln| ln.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .collect::<Vec<i64>>();

    let mut cache = HashMap::new();
    let result: i64 = data.iter().map(|&num| count(num, 75, &mut cache)).sum();
    println!("{}", result);
}
