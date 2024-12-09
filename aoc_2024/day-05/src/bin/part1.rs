fn main() {
    part_1();
}
fn part_1() {
    println!("{:?}",include_str!("./input1.txt")
        .lines()
        .map(|ln| {
            let prefix = "mul(";
            let sufix = ")";
            let ids: Vec<(usize, char)> =
                ln.char_indices().filter(|(_, c)| *c == ',').collect();
            let mut sum = 0;

            for (comma_id, _) in ids {
                let r_bound = ln[comma_id..].find(sufix).and_then(|x| Some(x + comma_id));
                let l_bound = ln[..comma_id].rfind(prefix).and_then(|x| Some(x + prefix.len()));

                if let (Some(lb), Some(rb)) = (l_bound, r_bound) {
                    let right = &ln[comma_id + 1..rb].parse::<u32>();
                    let left = &ln[lb..comma_id].parse::<u32>();
                    if let (Ok(l), Ok(r)) = (left, right) {
                        sum += *l * *r;
                    }
                }
            }
            sum
        })
        .fold(0, |acc,num| acc +num));
}
