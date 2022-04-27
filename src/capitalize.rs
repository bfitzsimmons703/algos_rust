#![allow(dead_code)]

// --- Directions
// Write a function that accepts a string.  The function should
// capitalize the first letter of each word in the string then
// return the capitalized string.
// --- Examples
//   capitalize('a short sentence') --> 'A Short Sentence'
//   capitalize('a lazy fox') --> 'A Lazy Fox'
//   capitalize('look, it is working!') --> 'Look, It Is Working!'

fn solution_1(s: &str) -> String {
    let words: Vec<&str> = s.split(' ').collect();
    let mut new_words = vec![];
    for word in words {
        let new_word = word[0..1].to_uppercase() + &word[1..];
        new_words.push(new_word);
    }

    new_words.join(" ")
}

pub fn capitalize(s: &str) -> String {
    solution_1(s)
}

#[cfg(test)]
mod tests {
    use crate::capitalize::capitalize;

    #[test]
    fn test_1() {
        assert_eq!(capitalize("hi there, how is it going?"), "Hi There, How Is It Going?")
    }

    #[test]
    fn test_2() {
        assert_eq!(capitalize("i love breakfast at bill miller bbq"), "I Love Breakfast At Bill Miller Bbq")
    }
}
