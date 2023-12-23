use std::collections::HashMap;
/*
      +-----+  +-----+         +-----+
Light | Box |  | Box |   ...   | Box |
----------------------------------------->
      |  0  |  |  1  |   ...   | 255 |
      +-----+  +-----+         +-----+
*/

fn main() {
    let input = include_str!("input.txt");
    let mut boxes: Vec<Vec<String>> = vec![Vec::new(); 256];
    let mut focal_lengths: HashMap<String, usize> = HashMap::new();

    for instruction in input.trim().split(',') {
        let mut parts = instruction.split('=');
        let label = parts.next().unwrap();

        if let Some(length) = parts.next() {
            let length: usize = length.parse().unwrap();
            let index = hash(label);

            if !boxes[index].contains(&label.to_string()) {
                boxes[index].push(label.to_string());
            }

            focal_lengths.insert(label.to_string(), length);
        } else {
            let label = &label[..label.len() - 1];
            let index = hash(label);
            boxes[index].retain(|x| x != label);
        }
    }

    let total: usize = boxes
        .iter()
        .enumerate()
        .map(|(box_number, box_)| {
            box_.iter().enumerate().fold(0, |acc, (lens_slot, label)| {
                acc + (box_number + 1) * (lens_slot + 1) * focal_lengths[label]
            })
        })
        .sum();

    println!("{}",total);
}

///2x For loops
/// ```
/// calc_total(&boxes, &focal_lengths)
fn calc_total(boxes: &Vec<Vec<String>>, focal_lengths: &HashMap<String, usize>) -> usize {
    let mut total: usize = 0;
    for (box_number, box_) in boxes.iter().enumerate() {
        for (lens_slot, label) in box_.iter().enumerate() {
            total += (box_number + 1) * (lens_slot + 1) * focal_lengths[label];
        }
    }
    total
}

fn hash(s: &str) -> usize {
    let mut v: usize = 0;
    for ch in s.chars() {
        v += ch as usize;
        v *= 17; //current value to itself multiplied by 17
        v %= 256; //current value to the remainder of dividing itself by 256
    }
    v
}
