fn main() {
    solution();
    // solution_2();
}
fn solution() {
    let input = include_str!("./input1.txt");
    let mut res = Vec::<u32>::new();

    input.lines().into_iter().for_each(|line| {
        let mut l: (usize, u32) = (0, 0); //ix,n
        let mut r: (usize, u32) = (0, 0);

        //left
        for (ix, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                l = (ix, c.to_digit(10).unwrap());
                break;
            }
        }
        //right
        for (ix, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                r = (ix, c.to_digit(10).unwrap());
                break;
            }
        }
        res.push(format!("{}{}", l.1, r.1).parse().unwrap());
    });
    println!("{:?}", res.iter().sum::<u32>());
}

fn solution_2() {
    let input = include_str!("./input1.txt");
    let mut res = Vec::<usize>::new();
    input.lines().for_each(|line| {
        let digits: Vec<(usize, usize)> = line
            .chars()
            .enumerate()
            .filter_map(|(ix, c)| c.to_digit(10).map(|n| (ix, n as usize)))
            .collect();

        let (l, r) = match digits.as_slice() {
            [] => ((0, 0), (0, 0)),
            [single] => (*single, *single),
            [first, .., last] => (*first, *last),
        };
        res.push(format!("{}{}", l.1, r.1).parse().unwrap());
    });
    println!("{:?}", res.iter().sum::<usize>());
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
