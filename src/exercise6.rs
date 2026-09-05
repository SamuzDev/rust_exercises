// PROBLEM: Read 500 numbers and get how many are positive.
// Testing adaptation: Receives a list of numbers and counts the positive ones (> 0).
pub fn count_positives(numbers: &[i32]) -> usize {
    numbers.iter().filter(|&&x| x > 0).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_positives() {
        assert_eq!(count_positives(&[-1, 0, 5, 10, -3]), 2);
    }
}
