pub fn vowels(s: String) -> usize {
    s.to_lowercase()
        .chars()
        .filter(|ch| match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false,
        })
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod vowels_test {
    use super::*;

    #[test]
    fn aeiou() {
        let ans = vowels("aeiou".to_string());
        let solution = 5;
        assert_eq!(solution, ans);
    }

    #[test]
    fn capital_aeiou() {
        let ans = vowels("AEIOU".to_string());
        let solution = 5;
        assert_eq!(solution, ans);
    }

    #[test]
    fn alphabet() {
        let ans = vowels("abcdefghijklmnopqrstuvwxyz".to_string());
        let solution = 5;
        assert_eq!(solution, ans);
    }

    #[test]
    fn random() {
        let ans = vowels("bcdfghjkl".to_string());
        let solution = 0;
        assert_eq!(solution, ans);
    }
}
