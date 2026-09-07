// PROBLEM: Exercise 16 - Asterisk Pyramid
// Generate a centered pyramid of asterisks ('*') with `n` levels.
// Return a Vector of Strings, where each String represents one level of the pyramid.
// The top level starts with 1 asterisk, and each subsequent level adds 2 more.
// You must add spaces to the left to keep it perfectly centered.
//
// Example for n = 3:
// [
//   "  *",
//   " ***",
//   "*****"
// ]
pub fn build_pyramid(n: usize) -> Vec<String> {
    (0..n).map(|i| {
        let spaces = " ".repeat(n - 1 - i);
        let asterisks  = "*".repeat(i * 2 + 1);
        format!("{}{}", spaces, asterisks)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_pyramid() {
        // Case with 0 levels (returns an empty vector)
        assert_eq!(build_pyramid(0), Vec::<String>::new());
        
        // Case with 3 levels
        assert_eq!(
            build_pyramid(3),
            vec![
                "  *",
                " ***",
                "*****"
            ]
        );

        // Case with 5 levels
        assert_eq!(
            build_pyramid(5),
            vec![
                "    *",
                "   ***",
                "  *****",
                " *******",
                "*********"
            ]
        );
    }
}