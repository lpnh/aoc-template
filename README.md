# Advent of Code 2023

A template for the [Advent of Code](https://adventofcode.com/) 2023.  
*Let the elves do the hard work for you!*

## Usage

### Solving

Update the `CURRENT_DAY` constant on the top of the file to match your progress.

```rust
const CURRENT_DAY: Day = Day::Day07; // Example
```

Write your solution to the Part One and Part Two in the `solve_part_1` and `solve_part_2` functions, respectively.

Note: The code compiles right from the start. This means you can try to test and solve the Part One without worrying about the Part Two.

### Testing (Optional)

With the provided examples, you can run a test to your solutions if you wish.

Inside the `tests` module, you can find the `check_part_1_solution` and `check_part_2_solution` functions.

Update `EXAMPLE_1` or `EXAMPLE_2` constant and the respective `EXPECTED_ANSWER_1` or `EXPECTED_ANSWER_2` to match the example for each part.

To test your solution, you can run `cargo test`, with the `--bin` flag to specify the day you want to test. For example, to test the Day 1:

```no_rust
cargo test --bin day_01
```

You can also run `cargo test --bins` to test all days at once.

### Getting the answer

Your first need to insert the puzzle input inside the `PUZZLE_INPUT` constant at the bottom of the file, if you haven't already.

Then, simple run `cargo run` with the `--bin` flag to specify the day you want to run. For example, to run the Day 1:

```no_rust
cargo run --bin day_01
```

The answer for each part will be stored inside the `solution.yaml` file in the root of the project.
It is not intended to be edited manually.

So you can just run `cat` or [bat](https://github.com/sharkdp/bat) to see all your answers in the terminal:

```no_rust
cat solution.yaml
```

### Next Days

I let a "default" `next_day.rs` file for the next days, so you can just copy and paste it in the same `bin` folder.  
You only need to rename it properly, like `day_02.rs`, etc.  
Don't forget to update the `CURRENT_DAY` constant in the new file.

---

🎄 Happy coding! 🎄
