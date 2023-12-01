use std::collections::HashMap;

struct Left {
    ix: usize,
    val: u32,
}
struct Right {
    ix: usize,
    val: u32,
}
fn main() {
    day_1_part2();
}
fn day_1_part2(){
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

    include_str!("./input1.txt")
        .lines()
        .into_iter()
        .for_each(|line| {
            let mut l: Left = Left { ix: 0, val: 0 };
            let mut r: Right = Right { ix: 0, val: 0 };

            for (ix, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    //no assignments on LEFT
                    if l.ix == 0 && l.val == 0 {
                        l = Left {
                            ix: ix,
                            val: c.to_digit(10).unwrap(),
                        };
                    }
                    //assign next trailing to RIGH
                    else {
                        r = Right {
                            ix: ix,
                            val: c.to_digit(10).unwrap(),
                        };
                    }
                }
            }
            //duplicate right if there is only one digit
            if r.val == 0 && l.val > 0 {
                r.ix = l.ix;
                r.val = l.val;
            }

            //build Sorted Vec<Tuple> by Index
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

            if l.val != 0 {
                sorted_tuple.push((l.ix, l.val));
            }
            if r.val != 0 {
                sorted_tuple.push((r.ix, r.val));
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