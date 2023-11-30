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