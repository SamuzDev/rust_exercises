// PROBLEM: Display odd numbers from 1 up to n (1, 3, 5, 7, 9, 11... n)
// Note: Returns a Vector with the numbers for easy testing.
pub fn get_odds(n: i32) -> Vec<i32> {
    (1..=n).filter(|&x| x & 1 == 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_odds() {
        assert_eq!(get_odds(11), vec![1, 3, 5, 7, 9, 11]);
    }
}
