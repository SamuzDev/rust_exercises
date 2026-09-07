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
    // TODO: Implementa tu lógica aquí
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_pyramid() {
        // Caso de 0 niveles (devuelve un vector vacío)
        assert_eq!(build_pyramid(0), Vec::<String>::new());
        
        // Caso de 3 niveles
        assert_eq!(
            build_pyramid(3),
            vec![
                "  *",
                " ***",
                "*****"
            ]
        );

        // Caso de 5 niveles
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