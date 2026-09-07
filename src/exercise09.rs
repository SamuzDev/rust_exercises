// PROBLEM: Find the maximum value in the dataset provided by the user.
// Note: Here numbers can be negative, we use Option in case the list is empty.
pub fn find_max_general(numbers: &[i32]) -> Option<i32> {
    numbers.iter()
        .copied()
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_general() {
        assert_eq!(find_max_general(&[-10, -2, -45]), Some(-2));
    }
}
