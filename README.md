# Advent of Code 2023 - Template

A template for the [Advent of Code](https://adventofcode.com/) 2023.

## Usage

Solve the problem using the `solution` function inside `part_1` and `part_2`.
You can run `cargo test` to check if your solution is correct.
Insert the example input inside the `test_.txt` and the expected output inside the `check_solution` function.

The puzzle input for each day should be inserted inside the `puzzle.txt` file.

To generate the answers for each day, run the following command:

```no_rust
cargo run <dayXX>
```

The answers will be stored inside the yaml file `solution.yaml` on the root.

Add new days inside the `src` folder, making the necessary changes.  
Don't forget to also add the module inside the `lib.rs` file.

## Visual Tree

.
├── Cargo.lock
├── Cargo.toml
├── README.md                   # you are here
├── solution.yaml               # your solution will be stored here
└── src
    ├── advent.rs               # nothing to see here
    ├── day_01
    │   ├── input
    │   │   ├── puzzle.txt      # insert the puzzle input here
    │   │   ├── test_1.txt      # insert the part 1 example input here (optional)
    │   │   └── test_2.txt      # insert the part 2 example input here (optional)
    │   ├── mod.rs              # nothing to see here
    │   ├── part_1.rs           # insert your solution here || insert the exemple's expected output (optional)
    │   └── part_2.rs           # insert your solution here || insert the exemple's expected output (optional)
    ├── lib.rs                  # add the new module here
    └── main.rs                 # add new days here
