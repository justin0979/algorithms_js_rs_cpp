fn palindrome(str: String) -> bool {}

#[cfg(test)]
mod palindrome_tests {
    use super::*;

    #[test]
    fn aba() {
        assert_eq!(palindrome("aba".to_string()), true);
    }

    #[test]
    fn aba_space() {
        assert_eq!(palindrome("aba ".to_string()), false);
    }

    #[test]
    fn space_aba() {
        assert_eq!(palindrome(" aba".to_string()), false);
    }

    #[test]
    fn greetings() {
        assert_eq!(palindrome("greetings".to_string()), false);
    }

    #[test]
    fn str_1000000001() {
        assert_eq!(palindrome("1000000001".to_string()), true);
    }

    #[test]
    fn capital_f_ish_space_hsif() {
        assert_eq!(palindrome("Fish hsif".to_string()), false);
    }

    #[test]
    fn pennep() {
        assert_eq!(palindrome("pennep".to_string()), true);
    }
}
