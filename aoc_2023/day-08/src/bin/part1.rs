use std::collections::BTreeMap;
use std::time::Instant;

fn main() {
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
    // println!("{:?}", hmap);

    let mut it = instructions.iter().cycle();
    let mut steps: u64 = 0;
    let ptr: String = "AAA".to_string();
    find_end(&mut it, &hmap, ptr, &mut steps);
    
    println!("steps {steps}");
}

fn find_end<'a>(
    it: &mut std::iter::Cycle<std::slice::Iter<'a, char>>,
    map: &BTreeMap<&str, (&str, &str)>,
    mut key: String,
    steps: &mut u64,
) {
    let start = Instant::now();
    while key != "ZZZ" {
        match it.next().unwrap() {
            'L' => {
                *steps += 1;
                let k = map.get(key.as_str()).unwrap().0;
                key = k.to_string(); // Update key for the next iteration
            }
            'R' => {
                *steps += 1;
                let k = map.get(key.as_str()).unwrap().1;
                key = k.to_string();
            }
            _ => panic!("NEVER"),
        }
    }
    let elapsed = start.elapsed();
    println!("ZZZ hit , elapsed {:?}", elapsed);
}

///In this function signature, 'a is a lifetime parameter that ensures the iterator’s lifetime is tied to the slice it’s iterating over.
///
/// The std::iter::Cycle type is a struct that wraps an iterator to make it cycle indefinitely.
///
/// The std::slice::Iter type is the iterator over a slice.
/// usage > _recursion_overflow(&mut it, &hmap, "AAA", &mut steps);
fn _recursion_overflow<'a>(
    it: &mut std::iter::Cycle<std::slice::Iter<'a, char>>,
    map: &BTreeMap<&str, (&str, &str)>,
    key: &str,
    steps: &mut u64,
) {
    if key == "ZZZ" {return;}
    
    match it.next().unwrap() {
        'L' => {
            *steps += 1;

            let k = map.get(key).unwrap().0;
            _recursion_overflow(it, map, k, steps);
        }
        'R' => {
            *steps += 1;

            let k = map.get(key).unwrap().0;
            _recursion_overflow(it, map, map.get(key).unwrap().1, steps);
        }
        _ => panic!("NEVER"),
    }
}