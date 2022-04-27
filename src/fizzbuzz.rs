#![allow(dead_code)]

// --- Directions
// Write a program that console logs the numbers
// from 1 to n. But for multiples of three print
// “fizz” instead of the number and for the multiples
// of five print “buzz”. For numbers which are multiples
// of both three and five print “fizzbuzz”.
// --- Example
//   fizzBuzz(5);
//   1
//   2
//   fizz
//   4
//   buzz
pub fn fizzbuzz(n: i32) -> String {
    let mut v: Vec<String> = Vec::new();

    for i in 1..=n {
        match i {
            x if x % 15 == 0 => v.push(String::from("fizzbuzz")),
            x if x % 3 == 0 => v.push(String::from("fizz")),
            x if x % 5 == 0 => v.push(String::from("buzz")),
            _ => v.push(i.to_string())
        }
    }
    
    v.join(",")
}

#[cfg(test)]
mod tests {
    use crate::fizzbuzz::fizzbuzz;

    #[test]
    fn test_1() {
        assert_eq!(fizzbuzz(15), "1,2,fizz,4,buzz,fizz,7,8,fizz,buzz,11,fizz,13,14,fizzbuzz");
    }
}
