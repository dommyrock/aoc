use std::process::Command;
use utils::timed_execution::TimedExecution;

fn main() {
    std::time::Instant::timed(|| {
        let output = Command::new("python")
            .arg("./src/bin/solve.py")
            .output()
            .expect("Failed to execute command");

        println!("Result:\n {}", String::from_utf8_lossy(&output.stdout));
    })
}

//NOT sure how to do in rust
//there is no lib accepting Symbols soo we can solve for Linear equatins as easy

/*
python mod ()
   https://www.sympy.org/en/index.html

Closest rust equivalent
   https://crates.io/crates/rulinalg
   https://crates.io/crates/nalgebra
   https://crates.io/crates/alga
*/

//Python solution can be exacuted
//1 cd .\aoc_2023\day-24
//2 python ./src/bin/python.py
