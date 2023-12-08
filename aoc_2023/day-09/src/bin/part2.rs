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
                    //b cmb a = DESC order
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
        .rev() //reverse so best cards get multiplied by bigger rank = idx
        .enumerate()
        .fold(0, |mut acc, (i, hand_bid)| {
            // if hand_bid.0.typ == HandType::FullHouse {
            println!(
                "Hand {:?} {} (bid) {} (i +1)",
                hand_bid.0,
                hand_bid.1,
                i + 1
            );
            // }
            acc += hand_bid.1 * (i as u32 + 1);
            acc
        });
    println!("Sum {}", res);
}

fn map_hand_type(hand: &str) -> Hand {
    use HandType::*; //bring variants into scope for convenience
    // if hand == "JJJJJ" {
    //     return Hand {
    //         typ: HighCard,
    //         char_vals: vec![0, 0, 0, 0, 0],
    //     };
    // }

    let counts = hand.replace("J", "").chars().counts();

    //check which char count is greatest and replace J wit that char
    let greatest_count_char = counts.iter().max_by(|x, y| {
        // First compare by the counts
        let count_cmp = x.1.cmp(&y.1);
        if count_cmp != std::cmp::Ordering::Equal {
            return count_cmp;
        }
        // If counts are equal, compare > Key
        let x_val = char_value(*x.0);
        let y_val = char_value(*y.0);
        // println!("A {x_val} {y_val}");
        x_val.cmp(&y_val) // We use x_val.cmp(&y_val) for ascending order
    });
    // //JJ222 ="222" was unwrap on none

    //TO LOW > 248814038 ,252186462

    //EDGECASE when we have two pair T55J66
    //and '5' =2 , '6'= 2 ,we replace with higher
    //so 55J66 -> 55666
    // println!("g_count char {:?}",greatest_count_char);

    let altered_counts = hand
        .chars()
        .map(|c| {
            if c == 'J' && greatest_count_char.is_some() {
                *greatest_count_char.unwrap().0
            } else {
                c
            }
        })
        .counts();

    let values = altered_counts.values().sorted().join("");
    let c_vals = hand.chars().map(char_value).collect::<Vec<usize>>();
    // println!("# {:?}", altered_counts);
    // println!("{values}");
    // println!();

    //TODO
    //T55J5  --> when we get type check if we can upgrade J to any other char to increase type , but keep cards the same after we assign type ?

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
        'J' => 0,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0,
    }
}
