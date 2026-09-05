// PROBLEM: Calculate and display the terms of the Fibonacci series: 0 1 1 2 3 5 8 13 21 34 55 89 ...... n
pub fn generate_fibonacci(limit: i32) -> Vec<i32> {
    // TODO: Your code here
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fibonacci() {
        assert_eq!(generate_fibonacci(13), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }
}