// PROBLEM: Generate, display, and calculate summations (you can structure internal functions)
// a. 1+2+3+4+5+6+...+N
pub fn summation_a(n: i32) -> i32 {
    (1..=n).sum()
}

// b. 1-2+3-4+5-6+...+N
pub fn summation_b(n: i32) -> i32 {
    (1..=n).map(|x| if x % 2 == 0 { -x } else { x }).sum()
}

// c. 1 + 1/2 + 1/3 + 1/4 + 1/5 + 1/6 + ... 1/N
pub fn summation_c(n: i32) -> f64 {
    (1..=n).map(|x| 1.0 / x as f64).sum()
}

// d. 1 - 1/2 + 1/3 - 1/4 + 1/5 - 1/6 + ... 1/N
pub fn summation_d(n: i32) -> f64 {
    (1..=n).map(|x| if x % 2 == 0 { -1.0 / x as f64 } else { 1.0 / x as f64 }).sum()
}

// e. 2-5+8-11+14-17+20-34+...+N (Note: Possible typo in the image "20-34", follow your logical sequence of adding 3 by 3)
pub fn summation_e(n: i32) -> i32 {
    (2..=n).step_by(3)
        .map(|x| if x % 2 == 0 { x } else { -x })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation_a() {
        assert_eq!(summation_a(4), 10); // 1 + 2 + 3 + 4
        assert_eq!(summation_a(1), 1);
    }

    #[test]
    fn test_summation_b() {
        assert_eq!(summation_b(3), 2);  // 1 - 2 + 3
        assert_eq!(summation_b(4), -2); // 1 - 2 + 3 - 4
    }

    #[test]
    fn test_summation_c() {
        assert_eq!(summation_c(1), 1.0); // 1
        assert_eq!(summation_c(2), 1.5); // 1 + 1/2
        
        // For f64 with repeating decimals, we verify that the difference is minimal
        assert!((summation_c(3) - 1.833333).abs() < 1e-5); // 1 + 1/2 + 1/3
    }

    #[test]
    fn test_summation_d() {
        assert_eq!(summation_d(1), 1.0); // 1
        assert_eq!(summation_d(2), 0.5); // 1 - 1/2
        assert!((summation_d(3) - 0.833333).abs() < 1e-5); // 1 - 1/2 + 1/3
    }

    #[test]
    fn test_summation_e() {
        // The sequence increments by 3: 2, 5, 8, 11...
        assert_eq!(summation_e(4), 2);   // The iterator only takes 2 (because 5 exceeds n=4)
        assert_eq!(summation_e(5), -3);  // 2 - 5
        assert_eq!(summation_e(8), 5);   // 2 - 5 + 8
        assert_eq!(summation_e(11), -6); // 2 - 5 + 8 - 11
    }
}