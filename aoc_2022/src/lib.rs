/*Run tests
cargo test -- --show-output
cargo test -- --nocapture
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader, Result},
        ops::RangeInclusive,
    };
    const INPUT_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2022\\src\\inputs";

    #[test]
    fn task3_2() -> Result<()> {
        let mut sum = 0;
        let (dict_l, dict_u) = get_hashsets([('a'..='z'), ('A'..='Z')]);

        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 3))?;
        let rows: Vec<String> = BufReader::new(f)
            .lines()
            .map(core::result::Result::unwrap)
            .collect();
        let groups = rows.chunks(3);

        for chunk in groups {
            let first: HashSet<char> = chunk.first().unwrap().chars().collect();
            for item in first {
                if let (Some(first), Some(_)) = (chunk[1].find(item), chunk[2].find(item)) {
                    let team_mark = chunk[1].chars().nth(first).unwrap();
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


//Sum by Descending Example
    max.sort();
    println!("{:?}", max);
    max.sort_by(|a, b| b.cmp(a));

//Converting numbers from string "1232" to numbers --> https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
    const HEX_RADIX: u32 = 16;
    let vecc = std::ops::RangeInclusive::new(1, 26);

//Maping range to Hashmap
    (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()

*/

// Bellow "\n\n" doesn't work on windows
// let lines = include_str!("inputs/1.txt").split("\n\n"); //OR std::fs::read_to_string("./src/inputs/1.txt").unwrap()
