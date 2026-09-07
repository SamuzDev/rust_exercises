# Rust Exercises

A collection of Rust programming exercises covering fundamental concepts, data structures, algorithms, and practical applications.

## Structure

Single crate with exercises as modules in `src/`:

```text
rust_exercises/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Exports all exercise modules
│   ├── exercise01.rs       # Odd numbers 1..n
│   ├── exercise02.rs       # Sum even numbers 2..100
│   ├── exercise03.rs       # Average of slice
│   ├── exercise04.rs       # Even numbers 20..10 descending
│   ├── exercise05.rs       # Multiples of 3 with filter
│   ├── exercise06.rs       # Count positive numbers
│   ├── exercise07.rs       # Odd/even sums tuple
│   ├── exercise08.rs       # Max of positive numbers
│   ├── exercise09.rs       # Max with negatives (Option)
│   ├── exercise10.rs       # Student struct, highest grade
│   ├── exercise11.rs       # Input validation (< 5)
│   ├── exercise12.rs       # Fibonacci sequence
│   ├── exercise13.rs       # Invoice with VAT & discount
│   ├── exercise14.rs       # Five summation variants
│   ├── exercise15.rs       # Factorial calculation
│   └── exercise16.rs       # Asterisk pyramid
```

## Usage

### Run Tests

```bash
# All tests
cargo test

# Specific exercise
cargo test exercise1

# With output
cargo test -- --nocapture
```

### Use as Library

```toml
# Cargo.toml
[dependencies]
rust_exercises = { path = "../rust_exercises" }
```

```rust
use rust_exercises::exercise01::get_odds;
use rust_exercises::exercise03::calculate_average;

fn main() {
    let odds = get_odds(11);        // [1, 3, 5, 7, 9, 11]
    let avg = calculate_average(&[10.0, 20.0, 30.0]);  // 20.0
}
```

## Exercises

| # | Module | Function | Description | Status |
| --- | -------- | ---------- | ------------- | -------- |
| 1 | `exercise01` | `get_odds(n)` | Odd numbers from 1 to n | [x] |
| 2 | `exercise02` | `sum_even_numbers_to_100()` | Sum of evens 2..100 | [x] |
| 3 | `exercise03` | `calculate_average(slice)` | Average of f64 slice | [x] |
| 4 | `exercise04` | `get_even_numbers_descending()` | Evens 20..10 descending | [x] |
| 5 | `exercise05` | `sum_filtered_multiples_of_three()` | Multiples of 3 (9..45), exclude 21..27 | [x] |
| 6 | `exercise06` | `count_positives(slice)` | Count positive numbers | [x] |
| 7 | `exercise07` | `calculate_odd_and_even_sums(slice)` | Tuple (odd_sum, even_sum) | [x] |
| 8 | `exercise08` | `find_max_positive(slice)` | Max of positives (0 if empty) | [x] |
| 9 | `exercise09` | `find_max_general(slice)` | Max with negatives (`Option<i32>`) | [x] |
| 10 | `exercise10` | `student_with_highest_grade(slice)` | Student ID with highest grade | [x] |
| 11 | `exercise11` | `validate_input_less_than_five(slice)` | First input < 5 (`Option<i32>`) | [x] |
| 12 | `exercise12` | `generate_fibonacci(limit)` | Fibonacci up to limit | [x] |
| 13 | `exercise13` | `calculate_invoice(price, qty)` | Price*qty + 16% VAT, 5% disc if >120k | [x] |
| 14 | `exercise14` | `summation_a..e(n)` | Five summation series | [x] |
| 15 | `exercise15` | `factorial(n)` | Factorial (recursive) | [x] |
| 16 | `exercise16` | `build_pyramid(n)` | Centered asterisk pyramid | [x] |
