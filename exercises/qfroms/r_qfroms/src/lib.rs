pub mod stack;
use stack::Stack;

pub struct Queue_from_stack<T> {}

#[cfg(test)]
mod queue_from_stack {
    use super::*;

    #[test]
    fn peeks() {
        let mut q = Queue_from_stack::<i32>::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), Some(&1));
    }

    #[test]
    fn pushes_and_pops() {
        let mut q = Queue_from_stack::<i32>::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), None);
    }
}
