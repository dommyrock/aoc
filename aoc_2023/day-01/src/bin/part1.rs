fn main() {
    let input = include_str!("./input1.txt");
    println!("{input}!");
}

fn test_fn(input:&str) -> String{
    "todo!()".to_string()
}

#[cfg(test)]
mod tests {
    use super::*; //use parent module items

    #[test]
    fn it_works() {
        let res = test_fn("");
        assert_eq!(res, "4".to_string());
    }
}
