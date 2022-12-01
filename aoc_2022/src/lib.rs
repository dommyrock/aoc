use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    //   | cargo test -- --show-output
    //or | cargo test -- --nocapture
};
const INPUT_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2022\\src\\inputs";

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn task2() {
        //ready
    }

    #[test]
    // #[ignore]
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
//     max.sort();
// println!("{:?}", max);
// max.sort_by(|a, b| b.cmp(a));
*/

// Bellow "\n\n" doesn't work on windows
// let lines = include_str!("inputs/1.txt").split("\n\n"); //OR std::fs::read_to_string("./src/inputs/1.txt").unwrap()

//------------------------------Utils------------------------------
pub fn read_input(i: i32) -> Result<Vec<i32>> {
    let mut out: Vec<i32> = vec![];

    let f = File::open(format!("{INPUT_PATH}\\{i}.txt"))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(res) = line?.parse::<i32>() {
            out.push(res);
        }
    }
    Ok(out)
}

pub fn read_input_str(i: i32) -> Result<Vec<String>> {
    let f = File::open(format!("{INPUT_PATH}\\{i}.txt"))?;
    let reader = BufReader::new(f);
    Ok(reader
        .lines()
        .map(|s| s.expect("failed to parse"))
        .collect::<Vec<String>>())
}
