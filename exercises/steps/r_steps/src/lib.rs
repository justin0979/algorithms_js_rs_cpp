pub fn steps(n: usize) -> Vec<String> {}

#[cfg(test)]
mod steps_test {
    use r_steps::steps;

    #[test]
    fn n_1() {
        let solution = vec!["#".to_string()];
        let result = steps(1);
        assert_eq!(solution, result);
    }

    #[test]
    fn n_2() {
        let solution = vec!["# ".to_string(), "##".to_string()];
        let result = steps(2);
        assert_eq!(solution, result);
    }

    #[test]
    fn n_3() {
        let solution = vec!["#  ".to_string(), "## ".to_string(), "###".to_string()];
        let result = steps(3);
        assert_eq!(solution, result);
    }
}
