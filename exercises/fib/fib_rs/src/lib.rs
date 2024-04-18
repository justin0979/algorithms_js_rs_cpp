pub fn fib(n: i32) -> i32 {
    let mut fib_seq: Vec<i32> = vec![0, 1];

    for i in 2..=n {
        fib_seq.push(fib_seq[(i - 1) as usize] + fib_seq[(i - 2) as usize]);
    }

    fib_seq[n as usize]
}

pub fn fib_recursive(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

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
