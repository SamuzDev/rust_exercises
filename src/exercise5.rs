// PROBLEM: Calculate the sum of multiples of 3 starting from 9 up to 45,
// excluding numbers between 21 and 27 from the sum.
pub fn sum_filtered_multiples_of_three() -> i32 {
    (9..=45)
        .step_by(3)
        .filter(|&x| !(21..=27).contains(&x))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_filtered_multiples_of_three() {
        // Modify the expected value according to your manual calculation
        assert_eq!(sum_filtered_multiples_of_three(), 279);
    }
}
