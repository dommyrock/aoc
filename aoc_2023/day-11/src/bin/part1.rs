#[derive(Debug)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn main() {
    //Expand galaxies horizontal
    let expanded: String = include_str!("input.txt")
        .to_string()
        .lines()
        .flat_map(|ln| {
            let mut new_lines = vec![ln.to_string()];
            if !ln.contains('#') {
                new_lines.push(".".repeat(ln.len()));
            }
            new_lines.into_iter()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let width = expanded.lines().next().unwrap().len();
    let mut lines: Vec<String> = expanded.lines().map(|line| line.to_string()).collect();

    //Expand vertically and map galaxy indexes
    let mut rows_without_galaxy: Vec<usize> = Vec::new();
    for x in 0..width {
        let mut contains_hash = false;
        for ln in &lines {
            if let Some(char) = ln.chars().nth(x) {
                if char == '#' {
                    contains_hash = true;
                    break;
                }
            }
        }
        if !contains_hash {
            rows_without_galaxy.push(x);
        }
    }
    //append vertical expansion
    lines.iter_mut().for_each(|ln| {
        let mut offset = 0;
        rows_without_galaxy.iter().for_each(move |&idx| {
            ln.insert(idx + offset, '.');
            offset += 1;
        })
    });
    //map galaxies
    let coords = lines
        .iter()
        .enumerate()
        .flat_map(|(y, ln)| {
            ln.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| Galaxy { x, y })
                .collect::<Vec<Galaxy>>()
        })
        .collect::<Vec<Galaxy>>();

    //Compute distances
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i + 1..coords.len() {
            let (g1, g2) = (&coords[i], &coords[j]);
            sum += grid_distance(g1.x as i32, g1.y as i32, g2.x as i32, g2.y as i32);
        }
    }
    println!("RESULT: {sum}");
}

fn grid_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}
