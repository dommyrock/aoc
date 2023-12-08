use itertools::Itertools;
use std::ops::Deref;

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug)]
struct Hand {
    typ: HandType,
    char_vals: Vec<usize>,
}

trait Sortable {
    ///sorts by 1st than 2nd than 3rd than 4th than 5th char val
    fn apply_sorters(&mut self);
}

impl Sortable for Vec<(Hand, u32)> {
    fn apply_sorters(&mut self) {
        self.sort_by(|a, b| {
            for (a_val, b_val) in a.0.char_vals.iter().zip(&b.0.char_vals) {
                match b_val.cmp(a_val) {
                    std::cmp::Ordering::Equal => continue, // If equal, move to the next element
                    non_equal => return non_equal,         // If not equal, determine the order
                }
            }
            std::cmp::Ordering::Equal // If all elements are equal, return Equal
        });
    }
}

fn main() {
    let mut five_of_kinds: Vec<(Hand, u32)> = Vec::new();
    let mut four_of_kinds: Vec<(Hand, u32)> = Vec::new();
    let mut full_houses: Vec<(Hand, u32)> = Vec::new();
    let mut three_of_kinds: Vec<(Hand, u32)> = Vec::new();
    let mut two_pairs: Vec<(Hand, u32)> = Vec::new();
    let mut one_pairs: Vec<(Hand, u32)> = Vec::new();
    let mut high_cards: Vec<(Hand, u32)> = Vec::new();

    include_str!("input.txt")
        .lines()
        .into_iter()
        .for_each(|ln| {
            let (l, r) = ln.split_once(" ").unwrap();
            let hand_type = map_hand_type(l);
            let bid = r.parse::<u32>().unwrap();
            let tuple = (hand_type, bid);
            match &tuple.0.typ {
                HandType::FiveOfAKind => five_of_kinds.push(tuple),
                HandType::FourOfAKind => four_of_kinds.push(tuple),
                HandType::FullHouse => full_houses.push(tuple),
                HandType::ThreeOfAKind => three_of_kinds.push(tuple),
                HandType::TwoPair => two_pairs.push(tuple),
                HandType::OnePair => one_pairs.push(tuple),
                HandType::HighCard => high_cards.push(tuple),
            }
        });

    let mut collections = [
        five_of_kinds,
        four_of_kinds,
        full_houses,
        three_of_kinds,
        two_pairs,
        one_pairs,
        high_cards,
    ];
    collections.iter_mut().for_each(|set| set.apply_sorters());

    let res = collections
        .iter()
        .flatten()
        .rev() //this way biger value cards get x bigger IDX
        .enumerate()
        .fold(0, |mut acc, (i, hand_bid)| {
            println!("{} (bid) {} (i +1)", hand_bid.1, i + 1);
            acc += hand_bid.1 * (i as u32 + 1);
            acc
        });
    println!("Sum {}", res);
}

fn map_hand_type(hand: &str) -> Hand {
    use HandType::*; //bring variants into scope for convenience

    let counts = hand.chars().counts();
    let values = counts.values().sorted().join("");
    let c_vals = hand.chars().map(char_value).collect::<Vec<usize>>();

    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        _ => panic!("should not happen {values}"),
    };

    Hand {
        typ: hand_type,
        char_vals: c_vals,
    }
}

fn char_value(c: char) -> usize {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}
