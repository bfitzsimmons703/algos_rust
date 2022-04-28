#![allow(dead_code)]

// --- Directions
// Print out the n-th entry in the fibonacci series.
// The fibonacci series is an ordering of numbers where
// each number is the sum of the preceeding two.
// For example, the sequence
//  [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
// forms the first ten entries of the fibonacci series.
// Example:
//   fib(4) === 3

fn solution_2(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => solution_2(n - 1) + solution_2(n - 2)
    }
}

fn solution_1(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut results = vec![0, 1];
    for i in 2..=n {
        let current = results.get(i - 1).unwrap() + results.get(i - 2).unwrap();
        results.push(current);
    }
    
    results[n]
}


pub fn fib(n: usize) -> usize {
    // solution_1(n)
    solution_2(n)
}

#[cfg(test)]
mod tests {
    use super::fib;

    #[test]
    fn test_1() {
        assert_eq!(fib(39), 63245986);
    }

    #[test]
    fn test_2() {
        assert_eq!(fib(1), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn test_4() {
        assert_eq!(fib(3), 2);
    }

    #[test]
    fn test_5() {
        assert_eq!(fib(4), 3);
    }
}
