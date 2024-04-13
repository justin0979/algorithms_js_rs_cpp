// Below method stores string into vector of char, sorts it, then counts chars until
// different char is encountered and starts count over; all the while, keeping track
// of both the char with the highest count and that highest count.

pub fn maxchar(s: String) -> char {}

#[cfg(test)]
mod maxchar {
    use super::*;

    #[test]
    fn abcdefghijklmnaaaaa_str() {
        let s = String::from("abcdefghijklmnaaaaa");
        let result = maxchar(s);
        let solution = 'a';

        assert_eq!(solution, result);
    }

    #[test]
    fn ab1c1d1e1f1g1_str() {
        let s = String::from("ab1c1d1e1f1g1");
        let result = maxchar(s);
        let solution = '1';

        assert_eq!(solution, result);
    }
}
