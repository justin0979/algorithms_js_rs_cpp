pub fn capitalize(s: String) -> String {}

#[cfg(test)]
mod capitalize_test {
    use r_capitalize::capitalize::capitalize;

    #[test]
    fn one() {
        let s = String::from("hi there, how is it going?");
        let result = capitalize(s);
        let solution = String::from("Hi There, How Is It Going?");

        assert_eq!(solution, result);
    }

    #[test]
    fn two() {
        let s = String::from("i love breakfast at bill miller bbq");
        let result = capitalize(s);
        let solution = String::from("I Love Breakfast At Bill Miller Bbq");

        assert_eq!(solution, result);
    }

    #[test]
    fn three() {
        let s = String::from("a bird, how is it a bird?");
        let result = capitalize(s);
        let solution = String::from("A Bird, How Is It A Bird?");

        assert_eq!(solution, result);
    }
}
