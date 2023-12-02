fn main() {
    solution();
}

fn solution() {
    let res: u32 = include_str!("./input.txt")
        .lines()
        .into_iter()
        .enumerate()
        .filter_map(|(ix, line)| {
            let id = ix + 1;
            let skip_head = 7 + id.to_string().len();
            let mut str_buffer: String = String::from("");
            let mut red_max: u32 = 0;
            let mut green_max: u32 = 0;
            let mut blue_max: u32 = 0;

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
                            red_max = red_max.max(digit);
                            str_buffer = "".to_string();
                        }
                        'b' => {
                            blue_max = blue_max.max(digit);
                            str_buffer = "".to_string();
                        }
                        'g' => {
                            green_max = green_max.max(digit);
                            str_buffer = "".to_string();
                        }
                        _ => (),
                    }
                }
            }
            Some(red_max * green_max * blue_max)
        })
        .collect::<Vec<u32>>()
        .iter()
        .sum();
    println!(" ----> sum {res}");
}
