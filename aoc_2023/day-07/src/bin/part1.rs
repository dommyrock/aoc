fn main() {
    let res = include_str!("input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .fold(
            Some(Data {
                times: Vec::new(),
                distances: Vec::new(),
            }),
            |acc, (i, l)| match i {
                0 => Some(Data {times: extract_data(l),distances: acc?.distances,}),
                1 => Some(Data {times: acc?.times,distances: extract_data(l),}),
                _ => acc,
            },
        )
        .map(|v| v)
        .and_then(|data| {
            let mut posible_ways: Vec<usize> = Vec::new();

            for (i, time) in data.times.iter().enumerate() {
                let dist_to_beat = data.distances[i];
                let mut count = 0;
                for j in 1..*time {
                    let dist = j * (time - j);
                    if dist > dist_to_beat {
                        count += 1;
                    }
                }
                posible_ways.push(count);
            }
            Some(posible_ways.iter().product::<usize>())
        })
        .unwrap();
      println!("{:?}", res);
}
fn extract_data(l: &str) -> Vec<usize> {
    l.split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}
struct Data {
    times: Vec<usize>,
    distances: Vec<usize>,
}
