fn main() {
    part_1();
}
fn part_1() {
    let check_range = |num: u32| (1..=3).contains(&num);
    let check_incr_or_decr = |nums: &[i32]| {
        nums.windows(2).all(|w| w[1] > w[0] && check_range(w[0].abs_diff(w[1]))) || 
        nums.windows(2).all(|w| w[1] < w[0] && check_range(w[0].abs_diff(w[1])))
    };
    println!("{:?}", include_str!("./input1.txt")
        .lines()
        .map(|ln| {
            let nums = ln
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            check_incr_or_decr(&nums)
        })
        .filter(|&x| x)
        .count());
}    