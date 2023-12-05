use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Digit {
    d: u32,
    ix: usize,
}
#[derive(Debug, Clone, Copy)]
struct Symb {
    ix: usize,
}
fn main() {
    solution();
}
/* indexes (nums & symbols)
1 skip 1st row
2 look previous vs current intersections
3 check if one is num and other is symbol
4 if true look left /right to concat chars into digit
5 add digit and summ it

012  567
   3
  23  678
      6
0123
     5 78
  234
      678
   3 5
 123 567
*/
fn solution() {
    let mut digits = Vec::<(usize, Vec<Digit>)>::new();
    let mut symbols = Vec::<(usize, Vec<Symb>)>::new();
    let mut id: usize = 1;
    include_str!("./input.txt")
        .lines()
        .into_iter()
        .for_each(|line| {
            let mut digit_set = Vec::<Digit>::new();
            let mut symb_set = Vec::<Symb>::new();
            for (i, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    digit_set.push(Digit {
                        d: c.to_string().parse::<u32>().unwrap(),
                        ix: i,
                    });
                }
                if c != '.' && !c.is_digit(10) {
                    symb_set.push(Symb { ix: i });
                }
            }
            digits.push((id, digit_set));
            symbols.push((id, symb_set));
            id += 1;
        });
    let mut intersects: Vec<(usize, HashSet<usize>)> = Vec::new();

    //noTE i might need to also to current digit / current symbol scan for 1st skipped row

    for (i, (_ix, curr_dig)) in digits.iter().enumerate().skip(1) {
        let prev_dig = &digits.get(i - 1).unwrap().1;
        let curr_symb = &symbols.get(i).unwrap().1;
        let prev_symb = &symbols.get(i - 1).unwrap().1;

        // let current_check = |d: &Digit, s: &Symb| d.ix == s.ix + 1 || d.ix == s.ix - 1;
        // let previous_check =
        // |d: &Digit, s: &Symb| d.ix == s.ix || d.ix == s.ix + 1 || d.ix == s.ix - 1;
        let current_check = |d: &Digit, s: &Symb| match d.ix {
            x if x == s.ix + 1 => Some(s.ix + 1),
            x if x == s.ix - 1 => Some(s.ix - 1),
            _ => None,
        };
        let previous_check = |d: &Digit, s: &Symb| match d.ix {
            x if x == s.ix + 1 => Some(s.ix + 1),
            x if x == s.ix - 1 => Some(s.ix - 1),
            x if x == s.ix => Some(s.ix),
            _ => None,
        };

        //1 compare current  numbers with previous symbols - >fn previous_check

        //2 check   current  symbols with previous nums ->fn previous_check

        //3 check current row nums and symbols  -> fn current_check

        //4 when there is match push that digit idx to intersects VEC

        let mut current_d_vs_prev_s: HashSet<usize> = curr_dig
            .iter()
            .filter_map(|d| {
                prev_symb
                    .iter()
                    .find(|s| previous_check(d, s).is_some())
                    .map(|_| d.ix)
            })
            .collect();
        let curr_symb_vs_prev_d: HashSet<usize> = curr_symb
            .iter()
            .filter_map(|s| {
                prev_dig
                    .iter()
                    .find(|d| previous_check(d, s).is_some())
                    .map(|x| x.ix)
            })
            .collect();
        // println!("dvs{:?}",current_d_vs_prev_s);
        // println!("svd {:?}",curr_symb_vs_prev_d);
        // println!();

        let curr_d_vs_curr_s: HashSet<usize> = curr_dig
            .iter()
            .filter_map(|d| {
                curr_symb
                    .iter()
                    .find(|s| current_check(d, s).is_some())
                    .map(|s| s.ix)
            })
            .collect();
        //merge prev
        current_d_vs_prev_s.extend(curr_symb_vs_prev_d);
        if current_d_vs_prev_s.len() > 0 {
            intersects.push((i, current_d_vs_prev_s));
        }
        if curr_d_vs_curr_s.len() > 0 {
            intersects.push((i + 1, curr_d_vs_curr_s));
        }
    }
    println!("{:?}", intersects);
}

// while current_idx > 7 {
//     //print bottom up

//     //compare on the same row (left /right check)
//     if let Some(d) = digits.get(current_idx) {
//         println!("D {} {:?}", d.0, d.1);
//     }
//     if let Some(s) = symbols.get(current_idx) {
//         println!("S {} {:?}", s.0, s.1);
//     }
//     println!();

//     //compare on preivious row (up / up +1 / up -1  )
//     let previous = current_idx - 1;
//     if let Some(d) = digits.get(previous) {
//         println!("D {} {:?}", d.0, d.1);
//     }
//     if let Some(s) = symbols.get(previous) {
//         println!("S {} {:?}", s.0, s.1);
//     }
//     current_idx -= 1;
// }

// fn solution_sum_while_iter() {
//     let mut current_d = Vec::<Digit>::new();
//     let mut current_sym_ix = Vec::<usize>::new();
//     let mut previous_d = Vec::<Digit>::new();
//     let mut previous_sym_ix = Vec::<usize>::new();
//     let mut sum: u32 = 0;

//     include_str!("./input.txt")
//         .lines()
//         .into_iter()
//         .for_each(|line| {
//             for (ix, c) in line.chars().enumerate() {
//                 //parse digit /  symbol into collections
//                 if c != '.' {
//                     if c.is_digit(10) {
//                         let d = Digit {
//                             d: c.to_string().parse::<u32>().unwrap(),
//                             ix: ix,
//                         };
//                         current_d.push(d);
//                     }
//                     if !c.is_digit(10) {
//                         current_sym_ix.push(ix);
//                     }
//                 }
//             }
//             //compare indexes current > previous & left right
//             //1 check for symbols next to nums on current
//             for dig in current_d.iter() {
//                 let left = current_sym_ix.get(dig.ix - 1).is_some();
//                 let right = current_sym_ix.get(dig.ix + 1).is_some();
//                 if left || right {
//                     // sum+=
//                     //we potentially add num here and again bellow
//                 }
//                 //check previous
//                 let left = previous_sym_ix.get(dig.ix - 1).is_some();
//                 let right = previous_sym_ix.get(dig.ix + 1).is_some();
//                 let above = previous_sym_ix.get(dig.ix).is_some();
//                 if left || right || above {
//                     // sum+=
//                 }
//             }

//             println!("{:?}", current_d);
//             println!("s {:?}", current_sym_ix);
//             println!("-------------previous");
//             println!("{:?}", previous_d);
//             println!("s {:?}", previous_sym_ix);
//             println!();

//             //SWAP before next iter
//             previous_d.clear();
//             previous_sym_ix.clear();
//             previous_d.append(&mut current_d);
//             previous_sym_ix.append(&mut current_sym_ix);
//             current_d.clear();
//             current_sym_ix.clear();
//         });
// }
