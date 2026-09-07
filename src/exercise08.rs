// PROBLEM: Find the maximum positive value in the dataset provided by the user.
// All numbers are positive. Return 0 if the list is empty.
pub fn find_max_positive(numbers: &[i32]) -> i32 {
    numbers.iter()
        .copied()
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_positive() {
        assert_eq!(find_max_positive(&[10, 50, 5, 23]), 50);
    }
}