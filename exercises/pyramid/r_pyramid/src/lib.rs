pub fn pyramid(n: usize) -> Vec<String> {}

#[cfg(test)]
mod pyramid_tests {
    use r_pyramid::pyramid;

    #[test]
    fn n_1() {
        let ans = pyramid(1);
        let solution = vec!["#".to_string()];
        assert_eq!(solution, ans);
    }

    #[test]
    fn n_2() {
        let ans = pyramid(2);
        let solution = vec![" # ".to_string(), "###".to_string()];
        assert_eq!(solution, ans);
    }

    #[test]
    fn n_3() {
        let ans = pyramid(3);
        let solution = vec![
            "  #  ".to_string(),
            " ### ".to_string(),
            "#####".to_string(),
        ];
        assert_eq!(solution, ans);
    }

    #[test]
    fn n_4() {
        let ans = pyramid(4);
        let solution = vec![
            "   #   ".to_string(),
            "  ###  ".to_string(),
            " ##### ".to_string(),
            "#######".to_string(),
        ];
        assert_eq!(solution, ans);
    }
}
