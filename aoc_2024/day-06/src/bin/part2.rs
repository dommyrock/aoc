use std::ops::Index;

fn main() {
    part_2();
}
fn part_2() {
    println!("{:?}",include_str!("./input1.txt")
        .lines()
        .map(|ln| {
            let prefix = "mul(";
            let sufix = ")";
            let commas: Vec<(usize, char)> =
                ln.char_indices().filter(|(_, c)| *c == ',').collect();
            let dont_s:Vec<usize>= ln.match_indices("don't()").map(|(id,_)|id +6).collect(); 
            let do_s:Vec<usize>= ln.match_indices("do()").map(|(id,_)|id+3).collect(); 

            let mut sum = 0;
            for (comma_id, _) in commas {
                let r_bound = ln[comma_id..].find(sufix).and_then(|x| Some(x + comma_id));
                let l_bound = ln[..comma_id].rfind(prefix).and_then(|x| Some(x + prefix.len()));
                
                let dont_before = dont_s.iter().filter(|&&dont_id| dont_id < comma_id ).last();
                let do_before = do_s.iter().filter(|&&do_id| do_id < comma_id ).last();
                //something doesn't work here i guess , either im not looking most recent do, don't

                match (dont_before,do_before){
                    // (None, None) => (),
                    // (None, Some(_)) => (),
                    (Some(_), None) => continue,
                    (Some(dont), Some(doo)) => {if dont > doo {continue;}},
                    _ => ()
                };

                if let (Some(lb), Some(rb)) = (l_bound, r_bound){
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
    //res2 61636489 to high
    //res2 52454448 to low
    //res1 174960292
}

//Cool bytes op solution
//REF: https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day03.rs