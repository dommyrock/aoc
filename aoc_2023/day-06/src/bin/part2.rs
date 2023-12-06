fn main() {
    let data: Data = include_str!("input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .fold(Data { time: 0, dist: 0 }, |acc, (i, l)| match i {
            0 => Data{time:extract_data(l),dist:acc.dist},
            1 => Data{time:acc.time, dist:extract_data(l)},
            _ => acc,
        });
    let mut count = 0;
    for i in 1..data.time {
        let dist_to_beat = data.dist;
        let dist = i * (data.time - i);
        if dist > dist_to_beat {
            count += 1;
        }
    }
    println!("{:?}", count);
}
fn extract_data(l: &str) -> usize {
    l.split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<usize>>()
        .iter()
        .fold("".to_string(), |acc: String, i| format!("{acc}{i}"))
        .parse::<usize>()
        .unwrap()
}
struct Data {
    time: usize,
    dist: usize,
}
