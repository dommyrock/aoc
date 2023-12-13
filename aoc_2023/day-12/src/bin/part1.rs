fn main() {
    let sum = include_str!("input.txt")
        .lines()
        .map(|ln| {
            let mut split = ln.split_whitespace();
            let config = split.next().unwrap();
            let nums = split
                .next()
                .unwrap()
                .split(',')
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            (config, nums)
        })
        .fold(0, |acc, t: (&str, Vec<u32>)| acc + count(t.0, t.1));
    println!("res {sum} ");
}

///Accounts for all possible outcomes 
fn count(config: &str, nums: Vec<u32>) -> u32 {
    println!("{config} {:?}",nums);//Explains it pretty well

    //base case 1
    if config.is_empty() {
        //strings left = 0 valid configs left / = one valid config
        return if nums.is_empty() { 1 } else { 0 };
    }
    //base case 2
    if nums.is_empty() {
        //if we have '#' in config but we were expecting None here
        //also if there are mupltiple ???? in config left all of them must become '.' = 1 valid case
        return if config.contains("#") { 0 } else { 1 };
    }
    
    let mut result = 0;
    //".?" if we see a dot or ? we can skip char and try next char
    if ".?".contains(config.chars().nth(0).unwrap()) {
        result += count(&config[1..], nums.clone());
    }

    //"#?" if we see # or ? being turned into #
    if "#?".contains(config.chars().nth(0).unwrap()) {
        let num_springs = nums[0] as usize;

        //IF there are enough springs left
        if nums[0] <= config.len() as u32
        //IF the first N springs are all broken
        && !config[..num_springs].contains(".")
        //next spring afterwards doesnt exist or we reached the END OF THE ROW
        && (nums[0] == config.len() as u32 
        //OR next one is operational to separate the blocks
        || config.chars().nth(num_springs).unwrap() != '#')
        {
            //cfg_suffix contains the characters of cfg from index nums[0] + 1 to the end
            let cfg_suffix: String = config.chars().skip(num_springs + 1).collect();
            result += count(&cfg_suffix, nums[1..].to_vec());
        }
    }
    result
}

/* NOTE: NUMS ARE IN ORDER AS EXPECTED IN INPUT
   https://en.wikipedia.org/wiki/Nonogram#Mathematical_approach
*/