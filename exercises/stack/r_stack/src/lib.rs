pub struct Stack<T> {}

#[cfg(test)]
mod stack_tests {
    use super::*;

    fn initialize<T>() -> Stack<T> {
        Stack::new()
    }

    #[test]
    fn stack_adds_and_removes_with_first_in_last_out() {
        let mut s: Stack<i32> = initialize();
        s.s_push(3);
        s.s_push(2);
        s.s_push(1);

        let mut result = Vec::new();
        for i in 0..3 {
            if let Some(val) = s.s_pop() {
                result.push(val);
            }
        }

        let expect = vec![1, 2, 3];

        assert_eq!(expect, result);
    }

    #[test]
    fn peeks() {
        let mut s: Stack<i32> = initialize();
        s.s_push(1);
        s.s_push(2);
        s.s_push(3);
        assert_eq!(Some(&3), s.peek());
        s.s_pop();
        assert_eq!(Some(&2), s.peek());
        s.s_pop();
        assert_eq!(Some(&1), s.peek());
        s.s_pop();
        assert_eq!(None, s.peek());
    }
}
