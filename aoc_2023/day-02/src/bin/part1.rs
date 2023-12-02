fn main() {
    solution();
}
fn solution() {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let res: u32 = include_str!("./input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .filter_map(|(ix, line)| {
            let id = ix + 1;
            let skip_head = 7 + id.to_string().len();
            let mut str_buffer: String = String::from("");
            for (idx, c) in line.chars().skip(skip_head).enumerate() {
                if c.is_digit(10) {
                    str_buffer.push(c);
                    continue;
                }
                //check if previous is whitespace
                if line
                    .chars()
                    .skip(skip_head)
                    .nth(idx - 1)
                    .unwrap()
                    .is_whitespace()
                {
                    let digit = str_buffer.parse::<u32>().unwrap();
                    match c {
                        'r' => {
                            let exceeds = RED.lt(&digit);
                            str_buffer = "".to_string();
                            if exceeds {
                                return None;
                            }
                        }
                        'b' => {
                            let exceeds = BLUE.lt(&digit);
                            str_buffer = "".to_string();
                            if exceeds {
                                return None;
                            }
                        }
                        'g' => {
                            let exceeds = GREEN.lt(&digit);
                            str_buffer = "".to_string();
                            if exceeds {
                                return None;
                            }
                        }
                        _ => (),
                    }
                }
            }
            Some(id as u32)
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    println!(" ----> sum {res}");
}

fn _solution_initial_miss() {
    const RED: u32 = 12;
    const GREEN: u32 = 13;
    const BLUE: u32 = 14;

    let res: u32 = include_str!("./input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .filter_map(|(ix, line)| {
            let id = ix + 1;
            let skip = 7 + id.to_string().len();
            let mut red_sum: u32 = 0;
            let mut green_sum: u32 = 0;
            let mut blue_sum: u32 = 0;
            let mut str_buffer: String = String::from("");
            for (idx, c) in line.chars().skip(skip).enumerate() {
                if c.is_digit(10) {
                    str_buffer.push(c);
                    continue;
                }
                //i also need to check if previous char was empty ' '
                if line
                    .chars()
                    .skip(skip)
                    .nth(idx - 1)
                    .unwrap()
                    .is_whitespace()
                {
                    match c {
                        'r' => {
                            red_sum += str_buffer.parse::<u32>().unwrap();
                            str_buffer = "".to_string();
                        }
                        'b' => {
                            blue_sum += str_buffer.parse::<u32>().unwrap();
                            str_buffer = "".to_string();
                        }
                        'g' => {
                            green_sum += str_buffer.parse::<u32>().unwrap();
                            str_buffer = "".to_string();
                        }
                        _ => (),
                    }
                }
            }

            //check if any sum > input_sum
            let is_valid = vec![red_sum, green_sum, blue_sum]
                .iter()
                .all(|sum| vec![RED, GREEN, BLUE].iter().all(|input| sum < input));
            if is_valid {
                Some(id as u32)
            } else {
                None
            }
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    println!(" ----> sum {res}");
}
