// --- Directions
// Check to see if two provided strings are anagrams of eachother.
// One string is an anagram of another if it uses the same characters
// in the same quantity. Only consider characters, not spaces
// or punctuation.  Consider capital letters to be the same as lower case
// --- Examples
//   anagrams('rail safety', 'fairy tales') --> True
//   anagrams('RAIL! SAFETY!', 'fairy tales') --> True
//   anagrams('Hi there', 'Bye there') --> False

use regex::Regex;

pub fn anagrams(mut s_one: String, mut s_two: String) -> bool {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_llohe() {
        let s_one = String::from("H!el!lo ?");
        let s_two = String::from("llohe");

        assert_eq!(anagrams(s_one, s_two), true);
    }

    #[test]
    fn one_one_two_two_two() {
        let s_one = String::from("one one");
        let s_two = String::from("two two two");

        assert_eq!(anagrams(s_one, s_two), false);
    }

    #[test]
    fn whoa_hi_hi_whoa() {
        let s_o = String::from("Whoa! Hi!");
        let s_t = String::from("Hi!, Whoa!");

        assert_eq!(anagrams(s_o, s_t), true);
    }

    #[test]
    fn one_one_one_one_c() {
        let s = String::from("One one");
        let t = String::from("One one c");

        assert_eq!(anagrams(s, t), false);
    }

    #[test]
    fn a_tree_a_life_a_bench() {
        let s = String::from("A tree, a life, a bench");
        let t = String::from("A tree, a fence, a yard");

        assert_eq!(anagrams(s, t), false);
    }
}
