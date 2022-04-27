#![allow(dead_code)]

// --- Directions
// Check to see if two provided strings are anagrams of eachother.
// One string is an anagram of another if it uses the same characters
// in the same quantity. Only consider characters, not spaces
// or punctuation.  Consider capital letters to be the same as lower case
// --- Examples
//   anagrams('rail safety', 'fairy tales') --> True
//   anagrams('RAIL! SAFETY!', 'fairy tales') --> True
//   anagrams('Hi there', 'Bye there') --> False

use std::collections::HashMap;

fn format_string(s: &str) -> String {
    let mut s = String::from(s);
    s.retain(|c| c.is_alphanumeric());
    s
}

//compare sorted strings method
fn solution_1(s1: &str, s2: &str) -> bool {
    let s1 = format_string(s1);
    let s2 = format_string(s2);

    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort_by(|a, b| b.cmp(a));
    s2_chars.sort_by(|a, b| b.cmp(a));

    String::from_iter(s1_chars) == String::from_iter(s2_chars)
}

fn build_char_map(s: String) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in s.chars() {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    map
}

//Character count method
fn solution_2(s1: &str, s2: &str) -> bool {
    let s1 = format_string(s1);
    let s2 = format_string(s2);

    let s1_char_map = build_char_map(s1);
    let s2_char_map = build_char_map(s2);

    if s1_char_map.keys().len() != s2_char_map.keys().len() {
        return false;
    }

    for (c, s1_count) in s1_char_map.iter() {
        if let Some(s2_count) = s2_char_map.get(c) {
            if s1_count != s2_count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

pub fn anagrams(s1: &str, s2: &str) -> bool {
    // solution_1(s1, s2)
    solution_2(s1, s2)
}

#[cfg(test)]
mod tests {
    use crate::anagrams::anagrams;

    #[test]
    fn test_1() {
        assert!(anagrams("hello", "llohe"));
    }

    #[test]
    fn test_2() {
        assert!(anagrams("Whoa! Hi!", "Hi! Whoa!"));
    }

    #[test]
    fn test_3() {
        assert!(!anagrams("One One", "Two two two"));
    }

    #[test]
    fn test_4() {
        assert!(!anagrams("One One", "One one c"));
    }

    #[test]
    fn test_5() {
        assert!(!anagrams("A tree, a life, a bench", "A tree, a fence, a yard"));
    }
}