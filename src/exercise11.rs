// PROBLEM: Write an algorithm that reads an integer from the keyboard and checks if it is less than 5.
// If not, it must read another number, repeating the operation until the user writes a correct value.
// Clean adaptation in Rust: We pass a list of "simulated inputs" and it returns the first one that is valid (< 5).
pub fn validate_input_less_than_five(simulated_inputs: &[i32]) -> Option<i32> {
    simulated_inputs.iter()
        .copied()
        .find(|&x| x < 5)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input_less_than_five() {
        // If it enters 8 (false), 6 (false), and then 3 (true), it should return 3.
        assert_eq!(validate_input_less_than_five(&[8, 6, 3]), Some(3));
    }
}