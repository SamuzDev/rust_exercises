// PROBLEM: Generate, display, and calculate summations (you can structure internal functions)
// a. 1+2+3+4+5+6+...+N
pub fn summation_a(n: i32) -> i32 {
    todo!()
}

// b. 1-2+3-4+5-6+...+N
pub fn summation_b(n: i32) -> i32 {
    todo!()
}

// c. 1 + 1/2 + 1/3 + 1/4 + 1/5 + 1/6 + ... 1/N
pub fn summation_c(n: i32) -> f64 {
    todo!()
}

// d. 1 - 1/2 + 1/3 - 1/4 + 1/5 - 1/6 + ... 1/N
pub fn summation_d(n: i32) -> f64 {
    todo!()
}

// e. 2-5+8-11+14-17+20-34+...+N (Note: Possible typo in the image "20-34", follow your logical sequence of adding 3 by 3)
pub fn summation_e(n: i32) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summation() {
        assert_eq!(summation_a(4), 10); // 1 + 2 + 3 + 4
        assert_eq!(summation_b(3), 2); // 1 - 2 + 3
    }
}