fn main() {
    part_2();
}
fn part_2() {
    println!("{:?}", include_str!("./input1.txt")
            .lines()
            .map(|ln| {
                let nums = ln
                    .split_whitespace()
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                is_safe(&nums)
            })
        .filter(|&x| x ==true)
        .count());
}
fn is_safe(nums: &[i32]) -> bool {
    let check_range = |num: u32| (1..=3).contains(&num);
    let check_incr_or_decr = |nums: &[i32]| {
        nums.windows(2).all(|w| w[1] > w[0] && check_range(w[0].abs_diff(w[1]))) || 
        nums.windows(2).all(|w| w[1] < w[0] && check_range(w[0].abs_diff(w[1])))
    };

    if  check_incr_or_decr(&nums)  {return true}

    for skip_idx in 0..nums.len() {
        let subset: Vec<i32> = nums
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != skip_idx)
            .map(|(_, &num)| num)
            .collect();

        if check_incr_or_decr(&subset) {return true}
    }
    false
}