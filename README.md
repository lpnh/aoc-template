# Advent of Code Template

A Rust template for the [Advent of Code](https://adventofcode.com/).  
*Let ~~Rust~~ elves do the hard work for you!*

## Usage

### Solving

Update the `CURRENT_DAY` and `PUZZLE_INPUT` constants on the top of the file to match your progress. Example:

```rust
const CURRENT_DAY: Day = Day::Day07;
const PUZZLE_INPUT: &str = include_str!("../../puzzle_input/day_07.txt");
```

Write your solution for Part One and Part Two in the `solve_part_1` and `solve_part_2` functions, respectively.

The function is expecting you to return `Result<String, anyhow::Error>`.
This means that you can return either `Ok(String)` or `Err(anyhow::Error)`.
The easiest way to achieve this is to simply wrap your solution with `Ok`:

```rust
fn answer_to_ultimate_question() -> Result<String, Error> {
    let result = "42".to_string();
    Ok(result)
}
```

and use `?` to propagate any potential errors:

```rust
fn answer_to_ultimate_question() -> Result<i32, Error> {
    let result = "42".parse::<i32>()?;
    Ok(result)
}
```

*The function name and signature are not intended to be changed.*

Note: The code compiles right from the start. This means you can try to test and solve the Part One without worrying about the Part Two.

### Testing (Optional)

With the provided examples, you can run a test for your solutions if you want to.

Inside the `tests` module, you can find the `check_part_1` and `check_part_2` functions.

Update `EXAMPLE_1` or `EXAMPLE_2` constant and the respective `EXPECTED_ANSWER_1` or `EXPECTED_ANSWER_2` to match the example for each part. Example:

```rust
fn check_part_1() {
    const EXAMPLE_1: &str = "
        example input for the part 1
        note: you can use indented
        multiline strings just like this.
        do as you wish,
        the elves will handle everything!
    ";

    const EXPECTED_ANSWER_1: &str = "";

    test_part_one!();
}
```

To test your solution, you can run `cargo test`, with the `--bin` flag to specify the day you want to test. For example, to test the Day 1:

```no_rust
cargo test --bin day_01
```

You can also run `cargo test --bins` to test all days at once.

Note: Both are passing by default. You can use them to test only the Part One, only the Part Two, or both.

### Getting The Answer

You first need to insert the puzzle input inside the `day_00.txt` file. You can find it inside the `puzzle_input` folder.

Then, use `cargo run` with the `--bin` flag to specify the day you want to run. For example, to run the Day 1:

```no_rust
cargo run --bin day_01
```

The answer will be printed in the terminal. Like this:

```no_rust

Day01 
---
Part One: 
Part Two: 

```

## Next Days

You can use the `day_00` as a default template for the next days. Just copy it and rename it accordingly.

## License

All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.

---

🎄 Happy coding! 🎄
