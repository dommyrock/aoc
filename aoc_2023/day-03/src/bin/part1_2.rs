use peeking_take_while::PeekableExt as _;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct PartNumber {
    num: u32,
    x: usize,
    y: usize,
    width: usize,
}
#[derive(Debug)]
struct Symbol {
    char: char,
    x: usize,
    y: usize,
}
impl PartNumber {
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        // we cover the cells [x, y]; [x + 1, y]; ...; [x + width; y]
        for x_offset in 0..self.width {
            if symbol.is_adjacent_to(self.x + x_offset, self.y) {
                return true;
            }
        }
        false
    }
}
impl Symbol {
    fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        let to_check = [
            (self.x.saturating_sub(1), self.y.saturating_sub(1)),
            (self.x.saturating_sub(0), self.y.saturating_sub(1)),
            (self.x.saturating_add(1), self.y.saturating_sub(1)),
            (self.x.saturating_sub(1), self.y.saturating_sub(0)),
            // don't need to check own tile
            (self.x.saturating_add(1), self.y.saturating_sub(0)),
            (self.x.saturating_sub(1), self.y.saturating_add(1)),
            (self.x.saturating_sub(0), self.y.saturating_add(1)),
            (self.x.saturating_add(1), self.y.saturating_add(1)),
        ];
        to_check.into_iter().any(|(tx, ty)| tx == x && ty == y)
    }
}

fn main() {
    let mut possible_part_numbers: Vec<PartNumber> = Vec::new();
    let mut symbols:Vec<Symbol> = Vec::new();

    for (y, line) in include_str!("input.txt").lines().enumerate() {
        
        let mut chars = line.chars().enumerate().peekable();
        while let Some((x, char)) = chars.next() {
            if char.is_ascii_digit() {
                let num_str = std::iter::once(char)
                    .chain(
                        chars.by_ref()
                            .peeking_take_while(|(_, chr)| chr.is_ascii_digit())
                            .map(|(_, chr)| chr),
                    )
                    .collect::<String>();

                let width = num_str.len();
                let num = num_str.parse::<u32>().unwrap();

                possible_part_numbers.push(PartNumber { num, x, y, width });
            } else if char != '.' {
                symbols.push(Symbol { x, y, char });
            }
        }
    }
    let mut part_numbers = HashSet::new();
    let mut gear_ratio_sum = 0;

    for symbol in symbols {
        let mut this_symbols_parts = Vec::new();

        for part_number in &possible_part_numbers {
            if part_number.is_adjacent_to(&symbol) {
                part_numbers.insert(part_number);
                this_symbols_parts.push(part_number);
            }
        }

        if symbol.char == '*' && this_symbols_parts.len() == 2 {
            gear_ratio_sum += this_symbols_parts
                .into_iter()
                .map(|pn| pn.num)
                .product::<u32>();
        }
    }
    println!(
        "Part numbers sum: {}",
        part_numbers.iter().map(|pn| pn.num).sum::<u32>()
    );//part1
    println!("Gear ratio sum: {gear_ratio_sum}");//part2
}

//cc https://github.com/Ununoctium117/aoc2023/blob/main/day3/src/main.rs
//cc https://github.com/tumdum/aoc2023/blob/main/src/day03.rs
