## AOC 2023

### Example commands

#### Run /Check sepeciffic binary

```shell
cd ./day-01

cargo check --bin part1

cargo run --bin part1
cargo run --bin part2
```

#### Build speciffic binary

```shell
cargo build --bin part1
```

#### Generate flamegraph for speciffic binary

```shell
cargo flamegraph --bin=part1 -o ../flamegraphs/day-01-part1.svg
```


### NOTE
Inputs files are excluded from git like seen in .gitignore > day-*/src/bin/input*

### 
> Day 14 has example of executing python code through rust 


#### Utils lib
Import 'utils' crate for each project you want to use it in.

```shell
[dependencies]
utils = { path = "../utils" }
```

```rust
// utils/src/lib.rs
pub mod timed_execution; //export for src/timed_execution.rs timer extension(closure wrapping executing fn())
```

```bash
cd ./aoc_2023/utils
tree ./utils

./utils
|-- Cargo.lock
|-- Cargo.toml
`-- src
    |-- lib.rs
    `-- timed_execution.rs
```
### tree CLI Download
1 cd C:\Program Files\Git\usr\bin 
2 paste tree.exe (after installation)

https://gnuwin32.sourceforge.net/packages/tree.htm