#[derive(Clone, Copy, Debug)]
pub struct Hailstone {
    sx: f64,
    sy: f64,
    sz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    pub fn new(sx: f64, sy: f64, sz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            sx: sx,
            sy: sy,
            sz: sz,
            vx: vx,
            vy: vy,
            vz: vz,
            a: vy,
            b: -vx,
            c: vy * sx - vx * sy,
        }
    }
}

fn main() {
    let hailstones = include_str!("input.txt")
        .lines()
        .map(|ln| {
            ln.replace("@", ",")
                .split(",")
                .map(|s| s.trim().to_string().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        })
        .map(|x| Hailstone::new(x[0], x[1], x[2], x[3], x[4], x[5]))
        .collect::<Vec<Hailstone>>();
    // for h in &hailstones {
    //     println!("{h}");
    // }

    let mut total = 0;
    for (i, hs1) in hailstones.iter().enumerate() {
        for hs2 in &hailstones[..i] {//up to i't element (..0) = empty[]
            let (a1, b1, c1) = (hs1.a, hs1.b, hs1.c);
            let (a2, b2, c2) = (hs2.a, hs2.b, hs2.c);
            //heilstones are parallel
            //a1/b1 = a2/b2 --> a1 * b2 = a2 * b1
            if a1 * b2 == a2 * b1 {
                continue;
            }
            //https://youtu.be/guOyA7Ijqgk?si=E2EBkThSUuI-cPGd&t=782
            //LInear equations
            let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1) as f64;
            let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1) as f64;
            // println!("{x} {y}");
            //Find interseting points by looking at Intersecting point and Starting point and see if they have he same prefix (- or + )
            if 200000000000000f64 <= x
                && x <= 400000000000000f64
                && 200000000000000f64 <= y
                && y <= 400000000000000f64
            {
                if [hs1, hs2]
                    .iter()
                    .all(|&hs| (x - hs.sx) * hs.vx >= 0.0 && (y - hs.sy) * hs.vy >= 0.0)
                {
                    total += 1;
                }
            }
        }
    }
    println!("{total}");
}

impl std::fmt::Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "HeilStone ({}, {}, {}, {}, {} ,{} | a,b,c {}, {}, {})",
            self.sx, self.sy, self.sz, self.vx, self.vy, self.vz, self.a, self.b, self.c
        )
    }
}
