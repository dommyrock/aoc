use std::collections::{HashMap, HashSet};
fn main() {
    let matches = include_str!("input.txt")
        .lines()
        .into_iter()
        .map(|ln| {
            let winners_guesses: Vec<String> = ln
                .chars()
                .skip(8)
                .collect::<String>()
                .split('|')
                .map(|s| s.to_string())
                .collect();
            let (w, me) = (winners_guesses[0].clone(), winners_guesses[1].clone());

            let winners: HashSet<usize> = get_nums(w).into_iter().collect();
            let guessers: HashSet<usize> = get_nums(me).into_iter().collect();
            let intersection: HashSet<_> = guessers.intersection(&winners).collect();
            intersection.len()
        })
        .collect::<Vec<usize>>();

    let mut card_count: Vec<u32> = vec![1; matches.len()];

    for card_id in 0..matches.len() {
        let card_matches = matches.get(card_id).unwrap();
        // for the number of matches, update any cards with copies.
        for m in 0..*card_matches {//= card copies to add
            card_count[card_id + 1 + m] += card_count[card_id];
        }
    }
    println!("{:?}",card_count.iter().sum::<u32>());
}

fn get_nums(s: String) -> Vec<usize> {
    s.split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

fn _recurse_fail(
    score: &mut usize,
    lookup: &Vec<usize>,
    card_id_queue: &mut Vec<usize>,
    memo: &mut HashMap<usize, Vec<usize>>,
) -> Vec<usize> {
    //bc
    if card_id_queue.len() == 0 {
        println!("final score{score}");
        return Vec::<usize>::new();
    }
    //get matches for passed card_ids
    let card_id = card_id_queue.remove(0);
    println!("got CARD ID {card_id}");

    if !memo.contains_key(&card_id) {
        let matches = lookup.get(card_id - 1).unwrap();
        let gen_range = (card_id + 1..=matches + card_id).collect::<Vec<usize>>();

        println!("{:?}", gen_range);

        memo.insert(card_id, gen_range.clone());
        *score += gen_range.len();

        if gen_range.len() > 0 {
            card_id_queue.extend(gen_range);
        }
        _recurse_fail(score, lookup, card_id_queue, memo);
    }
    let mem = memo[&card_id].clone();
    // *score += mem.len();
    // println!("id {card_id} {:?}", mem);
    mem
}
