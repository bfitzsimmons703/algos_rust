#![allow(dead_code)]

// --- Directions
// Given a string, return the character that is most
// commonly used in the string.
// --- Examples
// maxChar("abcccccccd") === "c"
// maxChar("apple 1231111") === "1"

use std::collections::HashMap;

pub fn max_char(s: &str) -> char {
    let mut max = 0;
    let mut max_char = s.chars().take(1).last().unwrap(); //default first character. Can assume the string will have length > 0
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    
    for c in s.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;

        if *count > max {
            max = *count;
            max_char = c;
        }
    }

    max_char
}

#[cfg(test)]
mod tests {
    use crate::maxchar::max_char;

    #[test]
    fn test_1() {
        assert_eq!(max_char("a"), 'a');
    }

    #[test]
    fn test_2() {
        assert_eq!(max_char("abcdefghijklmnaaaaa"), 'a');
    }

    #[test]
    fn test_3() {
        assert_eq!(max_char("ab1c1d1e1f1g1"), '1');
    }
}
