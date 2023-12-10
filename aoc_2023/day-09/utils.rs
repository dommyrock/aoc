// Helper function to calculate differences
fn abs_difference(seq: &[i32]) -> Vec<i32> {
   seq.windows(2).map(|w| (w[1] - w[0].abs())).collect()
}

//seq.iter().all() to check all are 00000

//NOTE Since i have pip3 i can use cargo to  prview this by callin dataviz through dot cli or calll python with rust to execute chart plots
//example inspired by https://arxiv.org/pdf/2306.15629v1.pdf

//py + r
//https://github.com/cpmech/plotpy
//https://github.com/matplotlib/matplotlib

//rust
//https://github.com/plotters-rs/plotters (2mo ago)
//https://github.com/askanium/rustplotlib (2yer ago)