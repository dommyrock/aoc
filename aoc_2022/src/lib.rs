/*Run tests
cargo test -- --show-output
cargo test -- --nocapture
*/

#[cfg(test)]
mod tests {
    // use super::*;
    use std::{
        collections::{BTreeSet, HashMap, HashSet, VecDeque},
        fs::File,
        hash::Hash,
        io::{BufRead, BufReader, Result},
        ops::RangeInclusive,
    };
    const INPUT_PATH: &str = "D:\\Me\\Git\\aoc\\aoc_2022\\src\\inputs";

    #[derive(Debug, Clone, Copy)]
    enum Command {
        Left,
        Right,
        Up,
        Down,
    }
    fn pos_to_enum(chr: char) -> Option<Command> {
        match chr {
            'L' => Some(Command::Left),
            'R' => Some(Command::Right),
            'U' => Some(Command::Up),
            'D' => Some(Command::Down),
            _ => None,
        }
    }
    fn map_to_list(element: &(Command, u32)) -> Vec<Command> {
        (0..element.1).into_iter().map(|_| element.0).collect()
    }
    #[test]
    // #[ignore]
    fn day9() -> Result<()> {
        let mut tail_positions: HashSet<u32> = HashSet::new();
        let mut commands: Vec<Command> = std::fs::read_to_string("./src/inputs/9.txt")? //1 Vec<(Command, u32)>
            .lines()
            .rev() //Rev so i can execute as in example by pop()
            .map(|ln| {
                let mut split = ln.split_whitespace();
                (
                    pos_to_enum(split.next().unwrap().parse::<char>().unwrap()).unwrap(),
                    split.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .flat_map(|x| map_to_list(&x))
            .collect();
            
        println!("count: {}", commands.len());
        commands.iter().for_each(|x| println!(": {:?}", x));


        Ok(())
    }

    #[test]
    #[ignore]
    fn day9_demo() -> Result<()> {
        //store positions only once with hashset
        // let mut tail_positions: HashSet<u32> = HashSet::new();
        let mut commands: Vec<Command> = std::fs::read_to_string("./src/inputs/9.txt")? //1 Vec<(Command, u32)>
            .lines()
            .rev() //Rev so i can execute as in example by pop()
            .map(|ln| {
                let mut split = ln.split_whitespace();
                (
                    pos_to_enum(split.next().unwrap().parse::<char>().unwrap()).unwrap(),
                    split.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .flat_map(|x| map_to_list(&x))
            .collect();
        //Print parsed data
        println!("count: {}", commands.len());
        commands.iter().for_each(|x| println!(": {:?}", x));

        //Print grid with executed commands
        let (w, h) = (9, 9);
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; w]; h];
        //state
        let (mut row, mut col) = (8,0);
        grid[row][col] = 's';
        while commands.len() > 0 {
            let cmd = commands.pop().unwrap();
            match cmd {
                Command::Left => {
                    col = col - 1;
                    grid[row][col] = 'L';
                }
                Command::Right => {
                    col = col + 1;
                    grid[row][col] = 'R';
                }
                Command::Up => {
                    row = row - 1;
                    grid[row][col] = 'U';
                }
                Command::Down => {
                    row = row + 1;
                    grid[row][col] = 'D';
                }
            }
        }
        for row in 0..h {
            for col in 0..w {
                let cur = grid[row][col];
                print!("{} ", cur);
            }
            println!("");
        }
        Ok(())
    }

    #[test]
    #[ignore]
    fn crete_and_mutate_2d_array() {
        let (w, h) = (29, 30);
        let mut grid: Vec<Vec<char>> = vec![vec!['.'; w]; h];
        // grid[14][14] = 's';

        for row in 0..h {
            for col in 0..w {
                let cur = grid[row][col];
                //test access----------
                if col == 3 || col == w - 4 {
                    print!("{} ", '3');
                    continue;
                }
                if row == 1 || row == h - 2 {
                    print!("{} ", '1');
                    continue;
                }
                if row == h - 1 && col == 0 {
                    grid[row][col] = 's';
                    print!("{} ", 'S');
                    continue;
                }
                //test access----------
                //default to '.'
                print!("{} ", cur);
            }
            println!("");
        }
    }

    #[test]
    #[ignore = "foreach"]
    fn range_foreach() {
        (0..4).for_each(|x| print!("{x} ,"));
        println!("");
        (0..=4).for_each(|x| print!("{x} ,"));
        let tr = (0..=4).any(|x| x * 2 == 8);
        print!("{} ,", tr)
    }

    #[test]
    #[ignore]
    fn day_8_2() -> Result<()> {
        let txt: String = std::fs::read_to_string("./src/inputs/8.txt")?;
        let y = txt.lines().count();
        let x = txt
            .split("\r\n")
            .into_iter()
            .take(1)
            .collect::<String>()
            .len();

        let mut grid = vec![vec![0; x]; y];
        //map to matrix
        let mut row = 0;
        for ln in txt.lines() {
            for (idx, c) in ln.chars().enumerate() {
                //map chars of eaxh line to grid
                grid[row][idx] = c.to_digit(10).unwrap();
            }
            row += 1;
        }

        let mut result = 0;
        for (i, row) in grid.iter().enumerate() {
            //exit @ las row
            if i == y - 1 {
                println!("Score {:?}", result);
                return Ok(());
            }
            for (j, _col) in row.iter().enumerate() {
                //iterate only inner parts
                if i >= 1 && j >= 1 && j < (x - 1) {
                    // print!("{:?},", current);
                    let mut horiz = (0..x).map(|f| grid[i][f]).collect::<Vec<u32>>();
                    let mut vert = (0..x).map(|f| grid[f][j]).collect::<Vec<u32>>();
                    //remove current
                    let _ = horiz.remove(j);
                    let _ = vert.remove(i);

                    let h_left: Vec<u32> = horiz.drain(0..j).collect();
                    let v_up: Vec<u32> = vert.drain(0..i).collect();

                    let current = grid[i][j];
                    //count  visible trees in up,left,bottom,right directions

                    let (mut l, mut r, mut u, mut d) = (0, 0, 0, 0);
                    //Runtime calculated func
                    let mut incre_while_less_or_eq = |f: u32, target: char| match target {
                        'l' => {
                            if f >= current {
                                l += 1;
                                return false;
                            }
                            l += 1;
                            return true;
                        }
                        'r' => {
                            if f >= current {
                                r += 1;
                                return false;
                            }
                            r += 1;
                            return true;
                        }
                        'u' => {
                            if f >= current {
                                u += 1;
                                return false;
                            }
                            u += 1;
                            return true;
                        }
                        'd' => {
                            if f >= current {
                                d += 1;
                                return false;
                            }
                            d += 1;
                            return true;
                        }
                        _ => false,
                    };
                    let _left = h_left.iter().rev().all(|f| incre_while_less_or_eq(*f, 'l'));
                    let _right = horiz.iter().all(|f| incre_while_less_or_eq(*f, 'r'));
                    let _up = v_up.iter().rev().all(|f| incre_while_less_or_eq(*f, 'u'));
                    let _down = vert.iter().all(|f| incre_while_less_or_eq(*f, 'd'));

                    //update larges
                    let total = (l * r * u * d);
                    if total > result {
                        result = total;
                    }
                }
            }
        }
        Ok(())
        //https://blog.logrocket.com/rust-iterators-closures-deep-dive/
        //https://stackoverflow.com/questions/65527921/modify-and-return-closure
    }

    #[test]
    #[ignore]
    fn day_8() -> Result<()> {
        let txt: String = std::fs::read_to_string("./src/inputs/8.txt")?;
        let y = txt.lines().count();
        let x = txt
            .split("\r\n")
            .into_iter()
            .take(1)
            .collect::<String>()
            .len();

        //or let mut grid = [[0u8; x]; y]; requires consts
        let mut grid = vec![vec![0; x]; y];
        //map to matrix
        let mut row = 0;
        for ln in txt.lines() {
            for (idx, c) in ln.chars().enumerate() {
                //map chars of eaxh line to grid
                grid[row][idx] = c.to_digit(10).unwrap();
            }
            row += 1;
        }

        let mut res: Vec<u32> = Vec::new();
        for (i, row) in grid.iter().enumerate() {
            //last iter row reached
            if i == y - 1 {
                println!("Exited @ j={}", i);
                println!(
                    "Invisible trees >{:?}, Result = {}",
                    res,
                    (x * y - res.len())
                );
                return Ok(());
            }
            // println!("i is ={}", grid[i][0]);
            for (j, _col) in row.iter().enumerate() {
                //iterate only inner parts
                if i >= 1 && j >= 1 && j < (x - 1) {
                    let mut horiz = (0..x).map(|f| grid[i][f]).collect::<Vec<u32>>();
                    let mut vert = (0..x).map(|f| grid[f][j]).collect::<Vec<u32>>();

                    //remove current
                    let _ = horiz.remove(j);
                    let _ = vert.remove(i);
                    let h_left: Vec<u32> = horiz.drain(0..j).collect();
                    let v_up: Vec<u32> = vert.drain(0..i).collect();

                    let current = grid[i][j];

                    let invisible_horiz = h_left.into_iter().any(|a| a >= current)
                        && horiz.into_iter().any(|a| a >= current);
                    let invisible_vert = v_up.into_iter().any(|a| a >= current)
                        && vert.into_iter().any(|a| a >= current);

                    println!("invis h[{invisible_horiz}], invis v[{invisible_vert}]");
                    if invisible_horiz && invisible_vert {
                        res.push(current);
                    }
                }
            }
            println!("");
        }
        Ok(())
        //https://stackoverflow.com/questions/13212212/creating-two-dimensional-arrays-in-rust
    }

    #[test]
    #[ignore]
    fn day7_v2() -> Result<()> {
        let lines: String = std::fs::read_to_string("./src/inputs/7.txt")?;
        let (mut cmds, mut children): (Vec<&str>, Vec<&str>) = (vec![], vec![]);
        //read the commands first , remove them from lines
        lines
            .split("\r\n")
            .skip(2)
            .into_iter()
            .enumerate()
            .for_each(|(i, ln)| {
                if ln.starts_with('$') {
                    cmds.push(ln);
                } else {
                    children.push(ln);
                }
            });
        cmds.iter().for_each(|c| println!("{:?}", c));
        println!("");
        children.iter().for_each(|c| println!("{:?}", c));

        /*
        Do the same as they did in bellow visual example
            MUTATE THE MAIN INPUT STRING WITH " " SPACING when i cd
            remove " " when cd ..
        */

        Ok(())
    }

    #[test]
    #[ignore]
    fn day7() -> Result<()> {
        //stack (directory, children) =child = everything after ls - all commands
        let mut stack: Vec<(&str, Vec<&str>)> = vec![
            // ("/", children)
        ];
        // let lines: Vec<&str> = ;
        let main: String = std::fs::read_to_string("./src/inputs/7.txt")?;

        let root_children = main
            .split("\r\n")
            .into_iter()
            .filter(|x| !x.starts_with('$'))
            .collect::<Vec<&str>>();
        stack.push(("/", root_children));

        println!("{:?}", stack);

        let cd_cmds: Vec<&str> = main
            .split("\r\n")
            .into_iter()
            .filter(|x| !x.starts_with("$ cd"))
            .collect();

        // recurse(std::fs::read_to_string("./src/inputs/7.txt")?
        // .split("\r\n")
        // .collect(),stack);

        Ok(())

        /*
        help
        https://stackoverflow.com/questions/44662312/how-to-filter-a-vector-of-custom-structs
        https://stackoverflow.com/questions/63011873/replace-if-condition-by-match
        */
    }
    fn recurse(lines: Vec<&str>, mut stack: Vec<(&str, Vec<&str>)>) {
        let cloned = lines.clone();
        // let cleaned =lines.iter().filter(|ln| )

        //list all children
        // println!("Cleaned: > {:?}",cleaned);
        //recurse

        ()
    }
    //v1 (missing depths and braincells)
    // let mut map: BTreeMap<String, Vec<i32>> = BTreeMap::new();
    // let mut cur_dir: Vec<String> = vec![];
    // for ln in std::fs::read_to_string("./src/inputs/7.txt")?.split("\r\n") {
    //     //dir abc or 123132 file.txt
    //     if !ln.starts_with('$') {
    //         if let Some(next) = ln.split_whitespace().next() {
    //             if let Ok(parsed) = next.parse::<i32>() {
    //                 map.entry(cur_dir.last().cloned().unwrap())
    //                     .or_insert(vec![])
    //                     .push(parsed);
    //             }
    //         }
    //     } else {
    //         //$ cd abc
    //         if !ln.starts_with("$ cd ..") && !ln.starts_with("$ ls") {
    //             let dir_name = ln.split_whitespace().last().unwrap();
    //             cur_dir.push(String::from(dir_name));
    //         }
    //     }
    // }
    // map.iter()
    //     .for_each(|(k, v)| println!("Map Key: {} Val sum: {:?}", k, v.iter().sum::<i32>()));
    // // let mut sorted_dir_sums: Vec<i32> = map
    // //     .values()
    // //     .map(|set| set.iter().sum::<i32>())
    // //     .into_iter()
    // //     .collect();
    // // sorted_dir_sums.sort();
    // // println!("Sorted: {:?}\n", sorted_dir_sums);
    // // let kkk = map
    // //     .values()
    // //     .map(|set| set.iter().sum::<i32>())
    // //     .filter(|x| *x <= 100_000)
    // //     .collect::<Vec<i32>>();
    // // for x in sorted_dir_sums {
    // //     if x <= 100_000 {
    // //         println!("x == {x}");
    // //         sol += x;
    // //     }
    // // }//1275893 To low

    #[test]
    #[ignore = "it can get x18 faster if we use single bit for storage?"]
    fn day6_v3_bitwise() -> Result<()> {
        let win_size = 14;
        let res = std::fs::read_to_string("./src/inputs/6.txt")?
            .as_bytes()
            .windows(win_size)
            .position(move |set| {
                let mut data: u32 = 0;
                for &c in set {
                    let prev = data;
                    data |= 1 << (c - b'a'); //b'a' same as 'a' as u8
                    if prev == data {
                        return false;
                    }
                }
                return true;
            });
        println!("Result idx => {:?}", res.unwrap() + win_size);
        Ok(())
    }
    #[test]
    #[ignore]
    fn day6_v2() -> Result<()> {
        let window_size = 14;
        std::fs::read_to_string("./src/inputs/6.txt")?
            .split("\r\n")
            .for_each(|line| {
                let chars: Vec<char> = line.chars().collect();
                let sequence = chars
                    .windows(window_size)
                    .enumerate()
                    // .inspect(|v| {dbg!(v);})
                    .find(|(_i, slice)| {
                        let set = slice.iter().collect::<BTreeSet<&char>>();
                        slice.len() == set.len()
                    })
                    .unwrap();
                println!("Result idx @ {}", (sequence.0 + window_size).to_string())
            });
        Ok(())
        //windows() demo: https://www.youtube.com/watch?v=pjbW_WvVx2Q
    }

    macro_rules! char_vec {
        ($text:expr) => {{
            let mut vec = Vec::new();
            let chars = $text.chars().collect::<Vec<char>>();
            for c in chars {
                vec.push(c);
            }
            vec
        }};
    }

    #[test]
    #[ignore]
    fn day6_1_2() -> Result<()> {
        let mut uniq: HashSet<char> = HashSet::new();
        let segment_size = 14; //(4 or 14)
        std::fs::read_to_string("./src/inputs/6.txt")?
            .split("\r\n")
            .for_each(|line| {
                for (i, _) in line.chars().enumerate() {
                    //look back N previous chars
                    if i >= (segment_size - 1) {
                        let x = char_vec!(line
                            .chars()
                            .skip(i - (segment_size - 1))
                            .take(segment_size)
                            .collect::<String>());

                        if all_unique_elements(x) {
                            println!("Resulting N index > {}", i + 1);
                            uniq.clear();
                            return;
                        }
                    }
                }
            });
        Ok(())
    }

    fn all_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    #[test]
    #[ignore = "macro test"]
    fn macro_test_2() {
        let taken_slice: String = "anabella".chars().skip(2).take(3).collect();
        println!(
            "T1 taken this {:?} String from slice input 'anabella'",
            taken_slice
        );

        let vec1 = char_vec!("bvwbjplbgvbhsrlpgdmjqwftvncz"
            .chars()
            .skip(4)
            .take(4)
            .collect::<String>());
        println!("T2 Out > [{:?}]", vec1);

        let vec2 = char_vec!("mjqjpqmgbljsphdztnvjfqwrcgsmlb"
            .chars()
            .skip(14)
            .take(14)
            .collect::<String>());
        println!("T2 Out > [{:?}]", vec2);
    }

    macro_rules! variable_inpt_test {
        ($($items:expr),+ =>$char_count:expr) => {{// i picked => to mark next macro param
            let mut vec = Vec::new();
            let mut counter = $char_count;
            while counter > 0
            {
                $(vec.push($items);)+ // + marks it as repetition on all input (items ) => (can also be * or , or ?)
                counter -=1;
            }
            vec
        }};
    }

    #[test]
    #[ignore = "macro test"]
    fn macro_test_variable_input() {
        //need to return (n) [char elements] from macro >Vec
        let res_char = variable_inpt_test!('1','2','3','4'=>2);
        let res_i32 = variable_inpt_test!(1,2,3,4=>2);
        let res_text = variable_inpt_test!("ananannas is great "=>2);

        println!("items {:?}", res_char);
        println!("items {:?}", res_i32);
        println!("items {:?}", res_text);
    }

    #[test]
    #[ignore]
    fn day6_bitwise_magic() {
        fn has_repeated_chars(s: &str) -> bool {
            println!("");

            let mut seen = 0;
            for c in s.chars() {
                let mask = 1 << (c as u8 - 'a' as u8);
                if (seen & mask) != 0 {
                    // This character has already been seen.
                    return true;
                }
                println!("(before |= mask) let seen = {}", seen);
                seen |= mask;
                println!("(after |= mask) let seen = {}", seen);
            }
            false
        }

        // Examples:
        assert!(has_repeated_chars("hello"));
        assert!(!has_repeated_chars("world"));
    }

    #[test]
    #[ignore]
    fn day5_1_2() -> Result<()> {
        let index_map: HashMap<usize, i32> = HashMap::from([
            (1, 1),
            (5, 2),
            (9, 3),
            (13, 4),
            (17, 5),
            (21, 6),
            (25, 7),
            (29, 8),
            (33, 9),
        ]);
        let mut third_map: HashMap<i32, VecDeque<char>> = HashMap::new();

        for (part, text_block) in std::fs::read_to_string("./src/inputs/5.txt")?
            .split("\r\n\r\n")
            .enumerate()
        {
            if part == 0 {
                //Add state to memory
                for line in text_block.lines() {
                    for (char_pos, chr) in line.chars().enumerate() {
                        if [' ', '[', ']'].iter().all(|x| *x != chr) && !chr.is_digit(10) {
                            third_map
                                .entry(index_map[&char_pos])
                                .or_insert(VecDeque::new())
                                .push_back(chr);
                        }
                    }
                }
            } else {
                let cmd_list = text_block
                    .lines()
                    .map(|ln| {
                        ln.split_whitespace()
                            .filter(|txt| txt.parse::<i32>().is_ok())
                            .map(|x| x.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>()
                    })
                    .collect::<Vec<Vec<i32>>>();

                //Update the state
                for cmd_set in cmd_list {
                    //p1
                    for _ in 0..cmd_set[0] {
                        if let Some(x) = third_map.get_mut(&cmd_set[1]) {
                            //add poped
                            if let Some(val) = x.pop_front() {
                                if let Some(v) = third_map.get_mut(&cmd_set[2]) {
                                    v.push_front(val);
                                }
                            }
                        }
                    }
                    //p2
                    // if let Some(x) = third_map.get_mut(&cmd_set[1]) {
                    //     let poped: VecDeque<char> =
                    //         x.drain(..cmd_set[0] as usize).collect::<VecDeque<char>>();
                    //     if let Some(ok) = third_map.get_mut(&cmd_set[2]) {
                    //         poped.iter().rev().for_each(|x| ok.push_front(*x));
                    //     }
                    // }
                }
                //Result state
                for (key, val) in &third_map {
                    println!("Key {}, Peek : [{:?}]", key, val.front());
                }
            }
        }
        Ok(())
    }

    #[test]
    #[ignore]
    fn day4_1_2() -> Result<()> {
        let (mut overlap2, mut overlap1) = (0, 0);
        let range_pairs: Vec<Vec<i32>> = include_str!("./inputs/4.txt")
            .split("\r\n")
            .map(|x| x.split(',').collect::<Vec<&str>>())
            .flat_map(|y| y)
            .map(|x| x.split('-').collect::<Vec<&str>>())
            .map(|v| {
                v.into_iter()
                    .map(|w| w.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        for pair in range_pairs.chunks(2) {
            let (lr_start, lr_end) = (pair[0].get(0).unwrap(), pair[0].get(1).unwrap());
            let (rr_start, rr_end) = (pair[1].get(0).unwrap(), pair[1].get(1).unwrap());

            if lr_start <= rr_end && lr_end >= rr_start {
                overlap2 += 1;
            }
            if [lr_start, lr_end]
                .iter()
                .all(|x| (rr_start..=rr_end).contains(x))
                || [rr_start, rr_end]
                    .iter()
                    .all(|x| (lr_start..=lr_end).contains(x))
            {
                overlap1 += 1;
            }
        }
        println!("p1 > {:?}", overlap1);
        println!("p2 > {:?}", overlap2);
        Ok(())
    }

    fn range_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
        let x: Box<dyn Iterator<Item = usize>>;
        if b > a {
            x = Box::new(a..=b)
        } else {
            x = Box::new((b..=a).rev())
        }
        x
    }

    #[test]
    #[ignore]
    fn testing_ranges_start_gt_end() {
        for i in range_inclusive(3, 1).zip(range_inclusive(1, 3)) {
            println!("{:?}", i);
        }
    }

    #[test]
    #[ignore]
    fn day3_2() -> Result<()> {
        let mut sum = 0;
        let (dict_l, dict_u) = get_hashsets([('a'..='z'), ('A'..='Z')]);

        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 3))?;
        let rows = BufReader::new(f)
            .lines()
            .map(core::result::Result::unwrap)
            .collect::<Vec<String>>();

        for chunk in rows.chunks(3) {
            let first_of_group: HashSet<char> = chunk.first().unwrap().chars().collect();
            for c in first_of_group {
                if let (Some(second), Some(_third)) = (chunk[1].find(c), chunk[2].find(c)) {
                    let team_mark = chunk[1].chars().nth(second).unwrap();
                    match team_mark.is_lowercase() {
                        true => sum += *dict_l.get(&team_mark).unwrap(),
                        false => sum += *dict_u.get(&team_mark).unwrap(),
                    }
                    continue;
                }
            }
        }
        println!("Part 2 sum > {}", sum); //2444
        Ok(())
    }

    #[test]
    #[ignore]
    fn day3() -> Result<()> {
        let mut score = 0;
        let (dict_l, dict_u) = get_hashsets([('a'..='z'), ('A'..='Z')]);

        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 3))?;
        for line in BufReader::new(f).lines() {
            let mapped: Vec<usize> = line?
                .chars()
                .map(|c| {
                    if c.is_lowercase() {
                        *dict_l.get(&c).unwrap()
                    } else {
                        *dict_u.get(&c).unwrap()
                    }
                })
                .collect();
            let half = mapped.len() / 2;

            let found: Vec<Option<usize>> = mapped[..half]
                .iter()
                .map(|n| {
                    if mapped[half..].iter().any(|v| *v == *n) {
                        Some(*n)
                    } else {
                        None
                    }
                })
                .collect();

            let ok: Vec<usize> = found
                .iter()
                .filter_map(|x| match x {
                    Some(_) => *x,
                    None => None,
                })
                .collect();

            if let Some(fst) = ok.first() {
                score += fst;
            }
        }
        println!("Part 1 Score > {}", score);
        Ok(())
    }

    fn get_hashsets(r: [RangeInclusive<char>; 2]) -> (HashMap<char, usize>, HashMap<char, usize>) {
        let mut dict_l: HashMap<char, usize> = HashMap::new();
        let mut dict_u: HashMap<char, usize> = HashMap::new();
        let (mut idx, mut idx2) = (1, 27);

        // a-z = 1-26
        for c in r.get(0).unwrap().clone() {
            dict_l.insert(c, idx);
            idx += 1;
        }
        // A-Z = 27-52
        for c in r.get(1).unwrap().clone() {
            dict_u.insert(c, idx2);
            idx2 += 1;
        }
        return (dict_l, dict_u);
    }

    #[test]
    #[ignore]
    fn day2_2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 2))?;
        let mut score = 0;
        for ln in BufReader::new(f).lines() {
            if let Some((op, me)) = ln?.split_once(' ') {
                match (op, me) {
                    ("A", "X") => score += 3 + 0,
                    ("A", "Y") => score += 1 + 3,
                    ("A", "Z") => score += 2 + 6,
                    ("B", "X") => score += 1 + 0,
                    ("B", "Y") => score += 2 + 3,
                    ("B", "Z") => score += 3 + 6,
                    ("C", "X") => score += 2 + 0,
                    ("C", "Y") => score += 3 + 3,
                    ("C", "Z") => score += 1 + 6,
                    _ => (),
                }
            }
        }
        println!("Score 2> {}", score);
        Ok(())
        //X means lose, Y means draw, and Z means win.
        /*C,Z =Scissors =3
        *A,X =Rock = 1
        *B,Y =Paper =2
        win =6 ,draw = 3, loss=0
        */
    }

    #[test]
    #[ignore]
    fn day2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 2))?;
        let mut score = 0;
        for ln in BufReader::new(f).lines() {
            if let Some((op, me)) = ln?.split_once(' ') {
                match (op, me) {
                    ("A", "X") => score += 1 + 3,
                    ("A", "Y") => score += 2 + 6,
                    ("A", "Z") => score += 3 + 0,
                    ("B", "X") => score += 1 + 0,
                    ("B", "Y") => score += 2 + 3,
                    ("B", "Z") => score += 3 + 6,
                    ("C", "X") => score += 1 + 6,
                    ("C", "Y") => score += 2 + 0,
                    ("C", "Z") => score += 3 + 3,
                    _ => (),
                }
            }
        }
        println!("Score > {}", score);
        Ok(())
    }
    #[test]
    #[ignore]
    fn day1_2_prime() -> Result<()> {
        let mut max = include_str!("./inputs/1.txt")
            .split("\r\n\r\n") //on linux it's \n\n
            .map(|x| {
                return x.lines().flat_map(str::parse::<usize>).sum::<usize>();
            })
            .collect::<Vec<usize>>();

        max.sort_by(|a, b| b.cmp(a));

        println!("max me daddy {:?}", max.into_iter().take(3).sum::<usize>());
        return Ok(());
    }

    #[test]
    #[ignore]
    fn day1_2() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 1))?;
        let reader = BufReader::new(f);

        let mut res = 0;
        let mut totals: Vec<u32> = vec![];
        for line in reader.lines() {
            if let Ok(parsed) = line?.parse::<u32>() {
                res += parsed;
            } else {
                totals.push(res);
                res = 0;
            }
        }
        totals.sort();
        println!("{:?}", totals.iter().rev().take(3).sum::<u32>());
        /*
        OR desc sort
         max.sort_by(|a, b| b.cmp(a));
        than =>  max.iter().take(3).sum::<u32>()
        */
        Ok(())
    }

    #[test]
    #[ignore]
    fn day1() -> Result<()> {
        let f = File::open(format!("{INPUT_PATH}\\{}.txt", 1))?;
        let reader = BufReader::new(f);
        let mut res = 0;
        let mut max = 0;
        for line in reader.lines() {
            if let Ok(parsed) = line?.parse::<i32>() {
                res += parsed;
            } else {
                if res > max {
                    max = res
                }
                res = 0;
            }
        }
        println!("Result ==> {}", max);
        Ok(())
    }
}
//------------------------------Notes---------------------------------
/*
Iterator    https://doc.rust-lang.org/std/iter/trait.Iterator.html
Collections https://doc.rust-lang.org/std/collections/index.html
Sorting     https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
FromStr     https://doc.rust-lang.org/std/str/trait.FromStr.html


\n = CR (Carriage Return) // Used as a new line character in Unix
\r = LF (Line Feed) // Used as a new line character in Mac OS
\n\r = CR + LF // Used as a new line character in Windows
(char)13 = \n = CR // Same as \n

//read all contents to memory
for ln in std::fs::read_to_string("C:\\Users\\dpolzer\\Downloads")?.lines() {

//Sum by Descending Example
    max.sort();
    println!("{:?}", max);
    max.sort_by(|a, b| b.cmp(a));

//Converting numbers from string "1232" to numbers --> https://doc.rust-lang.org/std/primitive.char.html#method.to_digit
    const HEX_RADIX: u32 = 16;
    let vecc = std::ops::RangeInclusive::new(1, 26);

//Maping range to Hashmap
    (1..5).map(|i| (i + i, i * i)).collect::<HashMap<_, _>>()

//Chan and Vec<_>.split
 strmax.map(|x| x.split('-')).chain(other);//chain is for iterators over 2x collections at once
.split('-'); could be called on Vec to conditionaly exclude items from it

*/

// Bellow "\n\n" doesn't work on windows
// let lines = include_str!("inputs/1.txt").split("\n\n"); //OR std::fs::read_to_string("./src/inputs/1.txt").unwrap()
