use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};
const BASE_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2022\\inputs";

pub fn read_input(i: i32) -> Result<Vec<i32>> {
    let mut out: Vec<i32> = vec![];

    let f = File::open(format!("{BASE_PATH}\\{i}.txt"))?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(res) = line?.parse::<i32>() {
            out.push(res);
        }
    }
    Ok(out)
}

pub fn read_input_str(i: i32) -> Result<Vec<String>> {
    let f = File::open(format!("{BASE_PATH}\\{i}.txt"))?;
    let reader = BufReader::new(f);
    Ok(reader
        .lines()
        .map(|s| s.expect("failed to parse"))
        .collect::<Vec<String>>())
}

#[cfg(test)]
mod tests {
    use super::*;


}
