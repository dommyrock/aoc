use std::collections::BTreeMap;
use std::time::Instant;
//Least common multiple
//https://youtu.be/znmPfDfsir8?si=zfxkzS85TXuouFin&t=51

fn main() {
    let start = Instant::now();
    let mut instructions: Vec<char> = Vec::new();
    let mut hmap: BTreeMap<&str, (&str, &str)> = BTreeMap::new();
    include_str!("input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .for_each(|(i, ln)| {
            if i == 0 {
                ln.chars().for_each(|c| instructions.push(c));
            } else if !ln.is_empty() {
                let key = ln.get(0..=2).unwrap();
                let val = (ln.get(7..=9).unwrap(), ln.get(12..=14).unwrap());
                hmap.insert(key, val);
            }
        });

    let total_steps = hmap
        .keys()
        .filter_map(|&x| if x.ends_with('A') { Some(x) } else { None })
        .collect::<Vec<&str>>()
        .iter()
        .map(|k| {
            let mut it = instructions.iter().cycle();
            let mut steps: u64 = 0;
            let total_steps = find_end(&mut it, &hmap, k.to_string(), &mut steps);
            total_steps
        })
        .collect::<Vec<u64>>()
        .chunks(2)
        .into_iter()
        .fold(1, |acc, x| {
            // Start with 1 since LCM of 1 and any number is the number itself
            num::integer::lcm(acc, num::integer::lcm(x[0], x[1]))
        });
    println!("{:?} - Elapsed {:?}", total_steps, start.elapsed());
}

fn find_end<'a>(
    it: &mut std::iter::Cycle<std::slice::Iter<'a, char>>,
    map: &BTreeMap<&str, (&str, &str)>,
    mut key: String,
    steps: &mut u64,
) -> u64 {
    while !key.ends_with('Z') {
        match it.next().unwrap() {
            'L' => {
                *steps += 1;
                let k = map.get(key.as_str()).unwrap().0;
                key = k.to_string(); // Update key for the next iteration
            }
            'R' => {
                *steps += 1;
                let k = map.get(key.as_str()).unwrap().1;
                key = k.to_string(); // Update key for the next iteration
            }
            _ => panic!("NEVER"),
        }
    }
    *steps
}
