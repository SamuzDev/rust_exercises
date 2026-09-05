// PROBLEM: Read 10 numbers and calculate:
// a. Sum of odd numbers
// b. Sum of even numbers
// Returns a tuple (odd_sum, even_sum)
pub fn calculate_odd_and_even_sums(numbers: &[i32]) -> (i32, i32) {
    numbers.iter().fold((0, 0), |(odd_sum, even_sum), &number| {
        if number & 1 == 1 {
            (odd_sum + number, even_sum)
        } else {
            (odd_sum, even_sum + number)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_odd_and_even_sums() {
        assert_eq!(calculate_odd_and_even_sums(&[1, 2, 3, 4]), (4, 6));
    }
}
