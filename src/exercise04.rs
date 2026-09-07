// PROBLEM: Display even numbers between 20 and 10 inclusive in descending order.
pub fn get_even_numbers_descending() -> Vec<i32> {
    (10..=20).rev().filter(|&x| x & 1 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_even_numbers_descending() {
        assert_eq!(get_even_numbers_descending(), vec![20, 18, 16, 14, 12, 10]);
    }
}
