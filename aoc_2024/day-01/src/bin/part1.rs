fn main() {
    part_1();
}
fn part_1() {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = include_str!("./input1.txt")
        .lines()
        .map(|ln| {
            let mut split = ln.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    println!("{}",
        left.iter().zip(right).fold(0, |acc, (left, right)| acc + left.abs_diff(right)));
}

fn test_fn(input: &str) -> String {
    format!("{input}").to_string()
}

#[cfg(test)]
mod tests {
    use super::*; //use parent module items

    #[test]
    fn it_works() {
        let results = vec![12, 38, 15, 77];
        let mut ix = 0;
        test_fn(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        )
        .lines()
        .into_iter()
        .for_each(|dig| {
            assert_eq!(dig, results[ix].to_string());
            ix += 1;
        });
    }
}
