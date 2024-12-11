fn main() {
    let mut nums = include_str!("./input1.txt")
        .lines()
        .flat_map(|ln| ln.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .collect::<Vec<i64>>();

    for _ in 0..25 {
        let mut interim = Vec::new();

        nums.iter().for_each(|num| match num {
            n if *n == 0 => interim.push(1),
            n if ((*n as f64).log10().floor() as usize + 1) % 2 == 0 => {
                let astr = n.to_string();
                let (l, r) = astr.split_at(astr.len() / 2);
                interim.extend_from_slice(&[
                    l.parse().unwrap_or_default(),
                    r.parse().unwrap_or_default(),
                ]);
            }
            _ => interim.push(*num * 2024),
        });
        nums = interim;
    }
    println!("{:?}", nums.len());
}
