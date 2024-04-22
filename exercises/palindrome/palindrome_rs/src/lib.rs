fn palindrome(str: String) -> bool {
    let str_arr = str.chars().collect::<Vec<char>>();
    let mut s_one: Vec<char> = (&str_arr[..str.len() / 2]).to_vec();
    s_one.reverse();
    let s_two: Vec<char>;

    if str.len() % 2 == 0 {
        s_two = (&str_arr[str.len() / 2..]).to_vec();
    } else {
        s_two = (&str_arr[(str.len() / 2) + 1..]).to_vec();
    }

    if s_one == s_two {
        return true;
    }

    false
}

#[cfg(test)]
mod palindrome_tests {
    use super::*;

    #[test]
    fn aba_str() {
        assert_eq!(palindrome("aba".to_string()), true);
    }

    #[test]
    fn space_aba_str() {
        assert_eq!(palindrome(" aba".to_string()), false);
    }

    #[test]
    fn aba_space_str() {
        assert_eq!(palindrome("aba ".to_string()), false);
    }

    #[test]
    fn greetings_str() {
        assert_eq!(palindrome("greetings".to_string()), false);
    }

    #[test]
    fn str_1000000001() {
        assert_eq!(palindrome("1000000001".to_string()), true);
    }

    #[test]
    fn fish_hsiF_str() {
        assert_eq!(palindrome("fish_hsiF".to_string()), false);
    }

    #[test]
    fn pennep_str() {
        assert_eq!(palindrome("pennep".to_string()), true);
    }
}
