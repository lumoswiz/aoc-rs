# Advent of Code

Solutions to [`adventofcode.com`](adventofcode.com) problems for 2018 in Rust.

## Running

To run all tests for all problems:
```
cargo test
```

To run tests for a specific day:
```sh
cargo test y${YEAR}::d${DAY}
```

To run with actual input:
```sh
export AOC_SESSION="<adventofcode.com session>"
cargo run -- ${YEAR} ${DAY}...
```
