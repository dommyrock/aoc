/*Run tests
cargo test -- --show-output
cargo test -- --nocapture
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::{HashMap, HashSet, VecDeque},
        fs::File,
        hash::Hash,
        io::{BufRead, BufReader, Result},
        ops::RangeInclusive,
    };
    const INPUT_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2022\\src\\inputs";

    macro_rules! batch_create_vec {
        ($text:expr) => {{
            let mut vec = Vec::new();
            let chars = $text.chars().collect::<Vec<char>>();
            for c in chars {
                vec.push(c);
            }
            vec
        }};
    }

    #[test]
    // #[ignore]
    fn task6_1_2() -> Result<()> {
        let mut uniq: HashSet<char> = HashSet::new();
        std::fs::read_to_string("./src/inputs/6.txt")?
            .split("\r\n")
            .for_each(|line| {
                for (i, _) in line.chars().enumerate() {
                    //look back N previous chars
                    let segment_size = 14; //(4 or 14)
                    if i >= (segment_size - 1) {
                        let x = batch_create_vec!(line
                            .chars()
                            .skip(i - (segment_size - 1))
                            .take(segment_size)
                            .collect::<String>());
                        if has_all_unique_elements(x) {
                            println!("Resulting N index > {}", i + 1);
                            uniq.clear();
                            return;
                        }
                    }
                }
            });
        Ok(())
    } 
    fn has_all_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    #[ignore = "macro test"]
    fn macro_test_2() {
        let taken_slice: String = "anabella".chars().skip(2).take(3).collect();
        println!(
            "T1 taken this {:?} String from slice input 'anabella'",
            taken_slice
        );

        let vec1 = batch_create_vec!("bvwbjplbgvbhsrlpgdmjqwftvncz"
            .chars()
            .skip(4)
            .take(4)
            .collect::<String>());
        println!("T2 Out > [{:?}]", vec1);

        let vec2 = batch_create_vec!("mjqjpqmgbljsphdztnvjfqwrcgsmlb"
            .chars()
            .skip(14)
            .take(14)
            .collect::<String>());
        println!("T2 Out > [{:?}]", vec2);
    }

    macro_rules! variable_inpt_test {
        ($($items:expr),+ =>$char_count:expr) => {{// i picked => to mark next macro param
            let mut vec = Vec::new();
            let mut counter = $char_count;
            while counter > 0
            {
                $(vec.push($items);)+ // + marks it as repetition on all input (items ) => (can also be * or , or ?)
                counter -=1;
            }
            vec
        }};
    }

    #[test]
    #[ignore = "macro test"]
    fn macro_test_variable_input() {
        //need to return (n) [char elements] from macro >Vec
        let res_char = variable_inpt_test!('1','2','3','4'=>2);
        let res_i32 = variable_inpt_test!(1,2,3,4=>2);
        let res_text = variable_inpt_test!("ananannas is great "=>2);

        println!("items {:?}", res_char);
        println!("items {:?}", res_i32);
        println!("items {:?}", res_text);
    }

    #[test]
    #[ignore]
    fn task6_bitwise_magic_test() {
        fn has_repeated_chars(s: &str) -> bool {
            println!("");

            let mut seen = 0;
            for c in s.chars() {
                let mask = 1 << (c as u8 - 'a' as u8);
                if (seen & mask) != 0 {
                    // This character has already been seen.
                    return true;
                }
                println!("(before |= mask) let seen = {}", seen);
                seen |= mask;
                println!("(after |= mask) let seen = {}", seen);
            }
            false
        }

        // Examples:
        assert!(has_repeated_chars("hello"));
        assert!(!has_repeated_chars("world"));
    }

    #[test]
    #[ignore]
    fn task5_1_2() -> Result<()> {
        let index_map: HashMap<usize, i32> = HashMap::from([
            (1, 1),
            (5, 2),
            (9, 3),
            (13, 4),
            (17, 5),
            (21, 6),
            (25, 7),
            (29, 8),
            (33, 9),
        ]);
        let mut third_map: HashMap<i32, VecDeque<char>> = HashMap::new();

        for (part, text_block) in std::fs::read_to_string("./src/inputs/5.txt")?
            .split("\r\n\r\n")
            .enumerate()
        {
            if part == 0 {
                //Add state to memory
                for line in text_block.lines() {
                    for (char_pos, chr) in line.chars().enumerate() {
                        if [' ', '[', ']'].iter().all(|x| *x != chr) && !chr.is_digit(10) {
                            third_map
                                .entry(index_map[&char_pos])
                                .or_insert(VecDeque::new())
                                .push_back(chr);
                        }
                    }
                }
            } else {
                let cmd_list = text_block
                    .lines()
                    .map(|ln| {
                        ln.split_whitespace()
                            .filter(|txt| txt.parse::<i32>().is_ok())
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>();

                //Update the state
                for cmd_set in cmd_list {
                    //p1
                    for _ in 0..cmd_set[0] {
                        if let Some(x) = third_map.get_mut(&cmd_set[1]) {
                            //add poped
                            if let Some(val) = x.pop_front() {
                                if let Some(v) = third_map.get_mut(&cmd_set[2]) {
                                    v.push_front(val);
                                }
                            }
                        }
                    }
                    //p2
                    // if let Some(x) = third_map.get_mut(&cmd_set[1]) {
                    //     let poped: VecDeque<char> =
                    //         x.drain(..cmd_set[0] as usize).collect::<VecDeque<char>>();
                    //     if let Some(ok) = third_map.get_mut(&cmd_set[2]) {
                    //         poped.iter().rev().for_each(|x| ok.push_front(*x));
                    //     }
                    // }
                }
                //Result state
                for (key, val) in &third_map {
                    println!("Key {}, Peek : [{:?}]", key, val.front());
                }
            }
        }
        Ok(())
    }

    #[test]
    #[ignore]
    fn task4_1_2() -> Result<()> {
        let (mut overlap2, mut overlap1) = (0, 0);
        let range_pairs: Vec<Vec<i32>> = include_str!("./inputs/4.txt")
            .split("\r\n")
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .flat_map(|y| y)
            .map(|x| x.split('-').collect::<Vec<&str>>())
            .map(|v| {
                v.into_iter()
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        for pair in range_pairs.chunks(2) {
            let (lr_start, lr_end) = (pair[0].get(0).unwrap(), pair[0].get(1).unwrap());
            let (rr_start, rr_end) = (pair[1].get(0).unwrap(), pair[1].get(1).unwrap());

            if lr_start <= rr_end && lr_end >= rr_start {
                overlap2 += 1;
            }
            if [lr_start, lr_end]
                .iter()
                .all(|x| (rr_start..=rr_end).contains(x))
                || [rr_start, rr_end]
                    .iter()
                    .all(|x| (lr_start..=lr_end).contains(x))
            {
                overlap1 += 1;
            }
        }
        println!("p1 > {:?}", overlap1);
        println!("p2 > {:?}", overlap2);
        Ok(())
    }

    fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
        let x: Box<dyn Iterator<Item = usize>>;
        if b > a {
            x = Box::new(a..=b)
        } else {
            x = Box::new((b..=a).rev())
        }
        x
    }

    #[test]
    #[ignore]
    fn testing_ranges_start_gt_end() {
        for i in range_inclusive(3, 1).zip(range_inclusive(1, 3)) {
            println!("{:?}", i);
        }
    }

    #[test]
    #[ignore]
    fn task3_2() -> Result<()> {
        let mut sum = 0;
        let (dict_l, dict_u) = get_hashsets([('a'..='z'), ('A'..='Z')]);

        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 3))?;
        let rows = BufReader::new(f)
            .lines()
            .map(core::result::Result::unwrap)
            .collect::<Vec<String>>();

        for chunk in rows.chunks(3) {
            let first_of_group: HashSet<char> = chunk.first().unwrap().chars().collect();
            for c in first_of_group {
                if let (Some(second), Some(_third)) = (chunk[1].find(c), chunk[2].find(c)) {
                    let team_mark = chunk[1].chars().nth(second).unwrap();
                    match team_mark.is_lowercase() {
                        true => sum += *dict_l.get(&team_mark).unwrap(),
                        false => sum += *dict_u.get(&team_mark).unwrap(),
                    }
                    continue;
                }
            }
        }
        println!("Part 2 sum > {}", sum); //2444
        Ok(())
    }

    #[test]
    #[ignore]
    fn task3() -> Result<()> {
        let mut score = 0;
        let (dict_l, dict_u) = get_hashsets([('a'..='z'), ('A'..='Z')]);

        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 3))?;
        for line in BufReader::new(f).lines() {
            let mapped: Vec<usize> = line?
                .chars()
                .map(|c| {
                    if c.is_lowercase() {
                        *dict_l.get(&c).unwrap()
                    } else {
                        *dict_u.get(&c).unwrap()
                    }
                })
                .collect();
            let half = mapped.len() / 2;

            let found: Vec<Option<usize>> = mapped[..half]
                .iter()
                .map(|n| {
                    if mapped[half..].iter().any(|v| *v == *n) {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .collect();

            let ok: Vec<usize> = found
                .iter()
                .filter_map(|x| match x {
                    Some(_) => *x,
                    None => None,
                })
                .collect();

            if let Some(fst) = ok.first() {
                score += fst;
            }
        }
        println!("Part 1 Score > {}", score);
        Ok(())
    }

    fn get_hashsets(r: [RangeInclusive<char>; 2]) -> (HashMap<char, usize>, HashMap<char, usize>) {
        let mut dict_l: HashMap<char, usize> = HashMap::new();
        let mut dict_u: HashMap<char, usize> = HashMap::new();
        let (mut idx, mut idx2) = (1, 27);

        // a-z = 1-26
        for c in r.get(0).unwrap().clone() {
            dict_l.insert(c, idx);
            idx += 1;
        }
        // A-Z = 27-52
        for c in r.get(1).unwrap().clone() {
            dict_u.insert(c, idx2);
            idx2 += 1;
        }
        return (dict_l, dict_u);
    }

    #[test]
    #[ignore]
    fn task2_2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 2))?;
        let mut score = 0;
        for ln in BufReader::new(f).lines() {
            if let Some((op, me)) = ln?.split_once(' ') {
                match (op, me) {
                    ("A", "X") => score += 3 + 0,
                    ("A", "Y") => score += 1 + 3,
                    ("A", "Z") => score += 2 + 6,
                    ("B", "X") => score += 1 + 0,
                    ("B", "Y") => score += 2 + 3,
                    ("B", "Z") => score += 3 + 6,
                    ("C", "X") => score += 2 + 0,
                    ("C", "Y") => score += 3 + 3,
                    ("C", "Z") => score += 1 + 6,
                    _ => (),
                }
            }
        }
        println!("Score 2> {}", score);
        Ok(())
        //X means lose, Y means draw, and Z means win.
        /*C,Z =Scissors =3
        *A,X =Rock = 1
        *B,Y =Paper =2
        win =6 ,draw = 3, loss=0
        */
    }

    #[test]
    #[ignore]
    fn task2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 2))?;
        let mut score = 0;
        for ln in BufReader::new(f).lines() {
            if let Some((op, me)) = ln?.split_once(' ') {
                match (op, me) {
                    ("A", "X") => score += 1 + 3,
                    ("A", "Y") => score += 2 + 6,
                    ("A", "Z") => score += 3 + 0,
                    ("B", "X") => score += 1 + 0,
                    ("B", "Y") => score += 2 + 3,
                    ("B", "Z") => score += 3 + 6,
                    ("C", "X") => score += 1 + 6,
                    ("C", "Y") => score += 2 + 0,
                    ("C", "Z") => score += 3 + 3,
                    _ => (),
                }
            }
        }
        println!("Score > {}", score);
        Ok(())
    }
    #[test]
    #[ignore]
    fn task1_2_prime() -> Result<()> {
        let mut max = include_str!("./inputs/1.txt")
            .split("\r\n\r\n") //on linux it's \n\n
            .map(|x| {
                return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
            })
            .collect::<Vec<usize>>();

        max.sort_by(|a, b| b.cmp(a));

        println!("max me daddy {:?}", max.into_iter().take(3).sum::<usize>());
        return Ok(());
    }

    #[test]
    #[ignore]
    fn task1_2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 1))?;
        let reader = BufReader::new(f);

        let mut res = 0;
        let mut totals: Vec<u32> = vec![];
        for line in reader.lines() {
            if let Ok(parsed) = line?.parse::<u32>() {
                res += parsed;
            } else {
                totals.push(res);
                res = 0;
            }
        }
        totals.sort();
        println!("{:?}", totals.iter().rev().take(3).sum::<u32>());
        /*
        OR desc sort
         max.sort_by(|a, b| b.cmp(a));
        than =>  max.iter().take(3).sum::<u32>()
        */
        Ok(())
    }

    #[test]
    #[ignore]
    fn task1() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 1))?;
        let reader = BufReader::new(f);
        let mut res = 0;
        let mut max = 0;
        for line in reader.lines() {
            if let Ok(parsed) = line?.parse::<i32>() {
                res += parsed;
            } else {
                if res > max {
                    max = res
                }
                res = 0;
            }
        }
        println!("Result ==> {}", max);
        Ok(())
    }
}
//------------------------------Notes---------------------------------
/*
Iterator    https://doc.rust-lang.org/std/iter/trait.Iterator.html
Collections https://doc.rust-lang.org/std/collections/index.html
Sorting     https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
FromStr     https://doc.rust-lang.org/std/str/trait.FromStr.html


\n = CR (Carriage Return) // Used as a new line character in Unix
\r = LF (Line Feed) // Used as a new line character in Mac OS
\n\r = CR + LF // Used as a new line character in Windows
(char)13 = \n = CR // Same as \n

//read all contents to memory
for ln in std::fs::read_to_string("C:\\Users\\dpolzer\\Downloads")?.lines() {

//Sum by Descending Example
    max.sort();
    println!("{:?}", max);
    max.sort_by(|a, b| b.cmp(a));

//Converting numbers from string "1232" to numbers --> https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
    const HEX_RADIX: u32 = 16;
    let vecc = std::ops::RangeInclusive::new(1, 26);

//Maping range to Hashmap
    (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()

//Chan and Vec<_>.split
 strmax.map(|x| x.split('-')).chain(other);//chain is for iterators over 2x collections at once
.split('-'); could be called on Vec to conditionaly exclude items from it

*/

// Bellow "\n\n" doesn't work on windows
// let lines = include_str!("inputs/1.txt").split("\n\n"); //OR std::fs::read_to_string("./src/inputs/1.txt").unwrap()
