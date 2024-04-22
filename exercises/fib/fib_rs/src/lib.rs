pub fn fib(n: i32) -> i32 {}

#[cfg(test)]
mod fib_tests {
    use super::*;
    #[test]
    fn one() {
        let expect = 1;
        let result = fib(1);
        assert_eq!(expect, result);
    }

    #[test]
    fn three() {
        let expect = 2;
        let result = fib(3);
        assert_eq!(expect, result);
    }

    #[test]
    fn four() {
        let expect = 3;
        let result = fib(4);
        assert_eq!(expect, result);
    }

    #[test]
    fn thirty_nine() {
        let expect = 63245986;
        let result = fib(39);
        assert_eq!(expect, result);
    }

    #[test]
    fn forty_two() {
        let expect = 267914296;
        let result = fib(42);
        assert_eq!(expect, result);
    }
}
