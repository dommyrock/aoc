use std::{collections::HashMap, time::Instant};
// EXACT SAME EXCEPT BECAUSE OF THE RECURSION DEPTH WE NEED TO MEMO RESULTS + x5 parsed inputs
fn main() {
    //*HASHMAP KEY has to be static / Owned  type = (String,Vec<u32>) to account for lifetimes
    Instant::timed(|| {

   let mut memo : HashMap<(String,Vec<u32>),u64> = HashMap::new();

   let sum = include_str!("input.txt")
       .lines()
       .map(|ln| {
           let mut split = ln.split_whitespace();
           let cfg =split.next().unwrap();
           let config_x5 = format!("{cfg}?").repeat(5);
           
           let nums = split
               .next()
               .unwrap()
               .split(',')
               .filter_map(|n| n.parse::<u32>().ok())
               .collect::<Vec<u32>>();
            let nums_x5 = std::iter::repeat(nums).take(5).flatten().collect();

           (config_x5[..config_x5.len() -1].to_string(), nums_x5)
       })
       .fold(0, |acc, t: (String, Vec<u32>)| acc + count(&t.0, t.1,&mut memo));
   println!("res {sum} ");
});

}

///Accounts for all possible outcomes 
fn count(config: &str, nums: Vec<u32>,cache: &mut HashMap<(String,Vec<u32>),u64>) -> u64 {
   //println!("{config} {:?}",nums);//Explains it pretty well

   //base case 1
   if config.is_empty() {
       //one valid config
       //strings left = 0 valid configs left
       return if nums.is_empty() { 1 } else { 0 };
   }
   //base case 2
   if nums.is_empty() {
       //if we have '#' in config but we were expecting None here
       //also if there are mupltiple ???? in config left all of them must become '.' = 1 valid case
       return if config.contains("#") { 0 } else { 1 };
   }
   let key =(config.to_owned(),nums.clone());

   if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
   }
   let mut result:u64 = 0;
   //".?" if we see a dot or ? we can skip char and try next char
   if ".?".contains(config.chars().nth(0).unwrap()) {
       result += count(&config[1..], nums.clone(), cache);
   }

   //"#?" if we see # or ? being turned into #
   if "#?".contains(config.chars().nth(0).unwrap()) {
       //IF there are enough springs left
       if nums[0] <= config.len() as u32
       //IF the first N springs are all broken
       && !config[..nums[0] as usize].contains(".")
       //3 next spring afterwards doesnt exist or we reached the END OF THE ROW
       && (nums[0] == config.len() as u32 
       //4 OR next one is operational to separate the blocks
       || config.chars().nth(nums[0] as usize).unwrap() != '#')
       {
           //cfg_suffix contains the characters of cfg from index nums[0] + 1 to the end
           let cfg_suffix: String = config.chars().skip(nums[0] as usize + 1).collect();
           result += count(&cfg_suffix, nums[1..].to_vec(), cache);
       }
   }
   cache.insert(key,result);
   
   result
}
trait TimedExecution {
    fn timed<T, F>(func: F) -> T
    where
        F: FnOnce() -> T;
}

impl TimedExecution for std::time::Instant {
    fn timed<T, F>(func: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start_time = std::time::Instant::now();
        let result = func();
        let elapsed = start_time.elapsed();
        println!("Elapsed: {:?}", elapsed);
        result
    }
}