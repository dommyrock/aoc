use std::collections::HashMap;
fn main() {
    let input = include_str!("./input1.txt");
    let mut res = Vec::<i32>::new();

    let hmap: HashMap<&str, u32> = HashMap::from_iter(vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    input.lines().into_iter().for_each(|line| {
        let mut l: (usize, u32) = (0, 0); //ix,n
        let mut r: (usize, u32) = (0, 0);

        for (ix, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if l.0 == 0 && l.1 == 0 {//no assignments on LEFT
                    l = (ix, c.to_digit(10).unwrap());
                } else { //assign next trailing to RIGH
                    r = (ix, c.to_digit(10).unwrap());
                }
            }
        }
        //duplicate right if there is only one digit
        if r.1 == 0 && l.1 > 0 {
            r.0 = l.0;
            r.1 = l.1;
        }

        //build Sorted Tuple by Index
        let mut sorted_tuple = Vec::<(usize, u32)>::new(); //(ix,num)
        hmap.iter().for_each(|(text, digit)| {
            //check front
            if let Some(found_ix) = line.find(text) {
                sorted_tuple.push((found_ix, *digit));
            }
            //check back
            if let Some(found_ix) = line.rfind(text) {
                sorted_tuple.push((found_ix, *digit));
            }
        });

        if l.1 != 0 {
            sorted_tuple.push((l.0, l.1));
        }
        if r.1 != 0 {
            sorted_tuple.push((r.0, r.1));
        }
        sorted_tuple.sort_by(|a, b| a.0.cmp(&b.0)); //ASC by idx

        let it = format!(
            "{}{}",
            sorted_tuple.first().unwrap().1,
            sorted_tuple.last().unwrap().1
        )
        .parse::<i32>()
        .unwrap();

        res.push(it);
    });
    println!("{:?}", res.iter().sum::<i32>());
}