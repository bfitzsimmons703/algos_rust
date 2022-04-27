#![allow(dead_code)]

// --- Directions
// Given an integer, return an integer that is the reverse
// ordering of numbers.
// --- Examples
//   reverseInt(15) === 51
//   reverseInt(981) === 189
//   reverseInt(500) === 5
//   reverseInt(-15) === -51
//   reverseInt(-90) === -9

pub fn reverse_int(n: i32) -> i32 {
    n.abs() //get rid of sign
        .to_string() //stringify it
        .chars() //iterator of chars 
        .rev() //reverse it
        .collect::<String>() //back to a string
        .parse::<i32>() //back to an i32
        .unwrap() //because parse() returns a Result
        * n.signum() //put back the sign
}

#[cfg(test)]
mod tests {
    use crate::reverseint::reverse_int;

    #[test]
    fn test_1() {
        assert_eq!(reverse_int(0), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_int(5), 5);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse_int(15), 51);
    }

    #[test]
    fn test_4() {
        assert_eq!(reverse_int(90), 9);
    }

    #[test]
    fn test_5() {
        assert_eq!(reverse_int(2359), 9532);
    }

    #[test]
    fn test_6() {
        assert_eq!(reverse_int(-5), -5);
    }

    #[test]
    fn test_7() {
        assert_eq!(reverse_int(-15), -51);
    }

    #[test]
    fn test_8() {
        assert_eq!(reverse_int(-90), -9);
    }

    #[test]
    fn test_9() {
        assert_eq!(reverse_int(-2359), -9532);
    }
}
