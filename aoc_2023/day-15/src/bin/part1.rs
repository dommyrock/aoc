fn main() {
    let input = include_str!("input.txt");
    let total: u32 = input.trim().split(',').map(|s| hash(s)).sum();
    println!("{}", total);
}
fn hash(s: &str) -> u32 {
    let mut v: u32 = 0;

    for ch in s.chars() {
        v += ch as u32;
        v *= 17; //current value to itself multiplied by 17
        v %= 256; //current value to the remainder of dividing itself by 256
    }
    v
}
