fn main() {
    part_2();
}

fn part_2() {
    let (left, right): (Vec<i32>, Vec<i32>) = include_str!("./input1.txt")
        .lines()
        .map(|ln| {
            let mut split = ln.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    println!("{}",
        right.iter().enumerate()
        .map(|(id, _)| left[id] * right.iter().filter(|r| **r == left[id]).count() as i32)
        .sum::<i32>()
    );
}
