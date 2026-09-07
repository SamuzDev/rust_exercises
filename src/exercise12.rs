// PROBLEM: Calculate and display the terms of the Fibonacci series: 0 1 1 2 3 5 8 13 21 34 55 89 ...... n
// pub fn generate_fibonacci(limit: i32) -> Vec<i32> {
//     std::iter::successors(Some((0, 1)), |&(a, b)| {
//         Some((b, a + b))
//     })
//     .map(|(a, _)| a)
//     .take_while(|&x| x <= limit)
//     .collect()
// }

// pub fn fibonacci_recursive(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     }

//     if n == 1 {
//         return 1;
//     }

//     fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
// }

pub fn generate_fibonacci(limit: i32) -> Vec<i32> {
    let mut result = Vec::new();
    
    if limit < 0 {
        return result;
    }

    let mut a = 0;
    let mut b = 1;

    while a <= limit {
        result.push(a);
        
        let next = a + b;
        a = b;
        b = next;
    }

    result
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_fibonacci() {
        assert_eq!(generate_fibonacci(13), vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    // #[test]
    // fn test_fib_rec() {
    //     // The Fibonacci term at position 6 (0, 1, 1, 2, 3, 5, 8) is 8
    //     assert_eq!(fibonacci_recursive(6), 8);
    // }
}