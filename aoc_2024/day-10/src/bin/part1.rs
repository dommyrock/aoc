fn main() {
    let mut nums = include_str!("./input1.txt")
        .lines()
        .flat_map(|ln| ln.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
        .collect::<Vec<i64>>();

   
}