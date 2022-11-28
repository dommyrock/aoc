use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};
const BASE_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2021\\inputs";

pub fn read_input(i: i32) -> Result<Vec<i32>> {
    let mut out: Vec<i32> = vec![];

    let f = File::open(format!("{BASE_PATH}\\{i}.txt"))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(res) = line?.parse::<i32>() {
            out.push(res);
        }
    }
    // println!("....retrieved input data\n");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1_2() {
        let data = read_input(1).unwrap();

        let sum = data.iter().enumerate().fold(0, |acc, (i, _)| {
            //Start comparing batches from 4th element
            if i > 2 && [data[i], data[i - 1], data[i - 2]].iter().sum::<i32>() > [data[i - 1], data[i - 2], data[i - 3]].iter().sum::<i32>() {
                acc + 1
            } else {
                acc
            }
        });

        println!("Result >>>> {}", sum);
        assert_eq!(data.len() > 0, true);
        
    }

    #[test]
    #[ignore]
    fn task1() {
        let data = read_input(1).unwrap();

        //logic
        let mut cnt = 0;
        for (i, el) in data.iter().enumerate() {
            if i > 0 && *el > data[i - 1] {
                cnt += 1;
            }
        }
        println!("Result >>>> {}", cnt);
        assert_eq!(data.len() > 0, true);
    }

    #[test]
    #[ignore]
    fn task1_v2() {
        let data = read_input(1).unwrap();

        let sum = data.iter().enumerate().fold(0, |acc, (i, el)| {
            if i > 0 && *el > data[i - 1] {
                acc + 1
            } else {
                acc
            }
        });

        println!("Result >>>> {}", sum);
        assert_eq!(data.len() > 0, true);
    }
}

//Print from tests
//cargo test -- --show-output
//cargo test -- --nocapture
//cargo test -- --help
//https://doc.rust-lang.org/book/ch11-02-running-tests.html#filtering-to-run-multiple-tests
//https://stackoverflow.com/questions/25106554/why-doesnt-println-work-in-rust-unit-tests

//Fold with index + element (chain enumerate wich gives index over items )
//https://stackoverflow.com/questions/41091641/is-there-a-way-to-fold-with-index-in-rust
//https://stackoverflow.com/questions/40836385/using-if-inside-of-fold
