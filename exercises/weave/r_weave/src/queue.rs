use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    data: VecDeque<T>,
    length: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: VecDeque::new(),
            length: 0,
        }
    }

    pub fn add(&mut self, record: T) {
        self.length += 1;
        self.data.push_front(record);
    }

    pub fn remove(&mut self) -> T {
        self.length -= 1;
        self.data.pop_back().unwrap()
    }

    pub fn peek(&self) -> Option<&T> {
        if self.length == 0 {
            return None;
        }
        self.data.get(self.data.len() - 1)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod queue_test {
    use super::*;

    #[test]
    fn adds() {
        let mut q: Queue<i32> = Queue::new();
        q.add(1);
        q.add(2);
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn removes() {
        let mut q: Queue<i32> = Queue::new();
        q.add(1);
        q.add(2);
        let one = q.remove();
        assert_eq!(q.len(), 1);
        assert_eq!(one, 1);
    }

    #[test]
    fn peeks() {
        let mut q: Queue<i32> = Queue::new();
        q.add(1);
        q.remove();
        q.add(2);
        assert_eq!(q.peek(), Some(&2));
    }
}
