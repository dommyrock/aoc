//This solution ends up being RING BUFFER APPROACH
use std::{
    collections::{HashMap, VecDeque},
    time,
};
use utils::timed_execution::TimedExecution;
fn main() {
    time::Instant::timed(|| {
        let grid: Vec<Vec<u32>> = include_str!("input.txt")
            .lines()
            .map(|ln| {
                ln.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        let mut buckets: VecDeque<Vec<(i32, i32, i32, i32, i32, i32)>> = VecDeque::new();

        (0..9).for_each(|_| {
            buckets.push_back(Vec::new());
        });

        buckets[0] = vec![(0, 0, 0, 0, 1, 0), (0, 0, 0, 1, 0, 0)];

        let mut seen: HashMap<(i32, i32, i32, i32, i32), i32> = HashMap::new();
        seen.insert((0, 0, 0, 1, 0), 0);
        seen.insert((0, 0, 1, 0, 0), 0);

        let target = ((grid.len() - 1) as i32, (grid[0].len() - 1) as i32);

        'exit: while !buckets.is_empty() {
            buckets.push_back(Vec::new());
            let bucket = buckets.pop_front().unwrap();

            for (hl, r, c, dr, dc, n) in bucket {
                if (r, c) == target && n >= 4 {
                    println!("{}", hl);
                    break 'exit;
                }

                let key = (r, c, dr, dc, n);
                if hl > *seen.get(&key).unwrap_or(&i32::MAX) {
                    continue;
                }

                let mut dirs = Vec::new();
                //we can move up to 10 times in the same dirr
                if n < 10 {
                    dirs.push((dr, dc));
                }
                //move at least 4 times in the same dir before we can turn
                if n >= 4 {
                    dirs.push((-dc, dr));
                    dirs.push((dc, -dr));
                }

                for (ndr, ndc) in dirs {
                    let nr = r + ndr;
                    let nc = c + ndc;
                    if 0 <= nr && nr < grid.len() as i32 && 0 <= nc && nc < grid[0].len() as i32 {
                        let nhl = hl + grid[nr as usize][nc as usize] as i32;
                        let key = (
                            nr,
                            nc,
                            ndr,
                            ndc,
                            if (ndr, ndc) == (dr, dc) { n + 1 } else { 1 },
                        );
                        if !seen.contains_key(&key) || nhl < *seen.get(&key).unwrap() {
                            seen.insert(key, nhl);
                            buckets[(grid[nr as usize][nc as usize] - 1) as usize]
                                .push((nhl, key.0, key.1, key.2, key.3, key.4));
                        }
                    }
                }
            }
        }
    });
}
//REf (pyhon) : https://www.youtube.com/live/dAptby54iDg?si=Rwf13s1aVgDhSYOc&t=9975
