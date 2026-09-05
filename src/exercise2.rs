// PROBLEM: Calculate and display the sum of even numbers from 2 to 100.
pub fn sum_even_numbers_to_100() -> i32 {
    (2..=100).step_by(2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_even_numbers_to_100() {
        assert_eq!(sum_even_numbers_to_100(), 2550);
    }
}
