// PROBLEM: Exercise 15 - Factorial
// Calculate the factorial of a given non-negative number N (denoted as N!).
// The factorial is the product of all positive integers less than or equal to N.
// Formula: N! = 1 * 2 * 3 * ... * N
// Special case: 0! = 1
// Example: 5! = 1 * 2 * 3 * 4 * 5 = 120

// pub fn factorial(n: u64) -> u64 {
//     (1..=n).product()
// }

pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 { return 1 };

    n * factorial(n - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);         // Caso especial
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);       // 5 * 4 * 3 * 2 * 1
        assert_eq!(factorial(10), 3_628_800);
    }
}