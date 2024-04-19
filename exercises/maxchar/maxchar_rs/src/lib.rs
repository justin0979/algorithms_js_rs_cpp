pub fn maxchar(s: String) -> char {
    let mut s_vec: Vec<char> = s.chars().collect::<Vec<char>>();
    s_vec.sort();

    let mut max_char: char = s_vec[0];
    let mut curr_char: char = s_vec[0];
    let mut max_count = 0;
    let mut counter = 0;

    for c in s_vec.into_iter() {
        if curr_char == c {
            counter += 1;
        } else {
            curr_char = c;
            counter = 1;
        }
        if max_count < counter {
            max_count = counter;
            max_char = c;
        }
    }

    max_char
}

#[cfg(test)]
mod maxchar {
    use super::*;

    #[test]
    fn a_str() {
        let s = String::from("a");
        let result = maxchar(s);
        let solution = 'a';

        assert_eq!(solution, result);
    }

    #[test]
    fn bzcdefghijklmnzzzzz_str() {
        let s = String::from("bzcdefghijklmnzzzzz");
        let result = maxchar(s);
        let solution = 'z';

        assert_eq!(solution, result);
    }

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

    #[test]
    fn cqqyyyabcabcddd123333zyyrrzyyqaayyyyyyyyyayz9_str() {
        let s = String::from("cqqyyyabcabcddd123333zyyrrzyyqaayyyyyyyyyayz9");
        let result = maxchar(s);
        let solution = 'y';

        assert_eq!(solution, result);
    }
}
