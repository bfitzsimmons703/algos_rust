#![allow(dead_code)]

// --- Directions
// Given a string, return a new string with the reversed
// order of characters
// --- Examples
//   reverse('apple') === 'leppa'
//   reverse('hello') === 'olleh'
//   reverse('Greetings!') === '!sgniteerG'

fn solution_1(s: &str) -> String {
    s.chars().rev().collect()
}

fn solution_2(s: &str) -> String {
    let mut chars = s.chars();
    let mut result = String::new();
    while let Some(c) = chars.next_back() {
        result.push(c);
    }
    result
}


pub fn reverse(s: &str) -> String {
    // solution_1(s)
    solution_2(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse("abcd"), "dcba");
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse("  abcd"), "dcba  ");
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse("Greetings!"), "!sgniteerG");
    }
}
