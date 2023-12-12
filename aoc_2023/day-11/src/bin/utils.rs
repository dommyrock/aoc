use std::fmt;
fn main(){}
#[derive(Debug, Clone, Copy)]
struct Galaxy {
    x: u64,
    y: u64,
}

impl fmt::Display for Galaxy {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Galaxy ({}, {})", self.x, self.y)
   }
}

/// n x (n-1) / 2
fn _get_pairs(n: u32) -> u32 {
   n * (n - 1) / 2
}
fn _cartesian_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
   ((x2 - x1).pow(2) as f32 + (y2 - y1).pow(2) as f32).sqrt() as i32
}

//https://www.cuemath.com/geometry/distance-between-two-points/
//d = √[(x2 − x1)^2 + (y2 − y1)^2]