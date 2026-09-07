// PROBLEM: Calculate and display the average of 50 numbers.
// Testing adaptation: Receives a slice of numbers (f64) and calculates the average.
pub fn calculate_average(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }

    numbers.iter().sum::<f64>() / numbers.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average() {
        let data = vec![10.0, 20.0, 30.0];
        assert_eq!(calculate_average(&data), 20.0);
    }
}
