#![allow(dead_code)]

// --- Directions
// Given a string, return true if the string is a palindrome
// or false if it is not.  Palindromes are strings that
// form the same word if it is reversed. *Do* include spaces
// and punctuation in determining if the string is a palindrome.
// --- Examples:
//   palindrome("abba") === true
//   palindrome("abcdefg") === false

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn solution_1(s: &str) -> bool {
    s == reverse_string(s)
}

fn solution_2(s: &str) -> bool {
    let mut chars = s.chars();
    
    while let Some(head) = chars.next() {
        if let Some(tail) = chars.next_back() {
            if head != tail {
                return false;
            }
        }
    }

    true
}

pub fn palindrome(s: &str) -> bool {
    //solution_1(s)
    solution_2(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(palindrome("aba"));
    }

    #[test]
    fn test_2() {
        assert!(!palindrome(" aba"));
    }

    #[test]
    fn test_3() {
        assert!(!palindrome("aba "));
    }

    #[test]
    fn test_4() {
        assert!(!palindrome("greetings"));
    }

    #[test]
    fn test_5() {
        assert!(palindrome("1000000001"));
    }

    #[test]
    fn test_6() {
        assert!(!palindrome("Fish hsif"))
    }

    #[test]
    fn test_7() {
        assert!(palindrome("pennep"))
    }
}