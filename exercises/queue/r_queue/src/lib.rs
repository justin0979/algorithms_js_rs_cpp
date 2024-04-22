use std::collections::VecDeque;

pub struct Queue {
    arr: VecDeque<i32>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            arr: VecDeque::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.arr.len()
    }

    pub fn add(&mut self, n: i32) {
        self.arr.push_back(n);
    }

    pub fn remove(&mut self) -> Option<i32> {
        self.arr.pop_front()
    }
}

#[cfg(test)]
mod queue_tests {
    use super::*;

    #[test]
    fn adds() {
        let mut q = Queue::new();
        q.add(1);
        assert_eq!(1, q.len());
    }

    #[test]
    fn removes() {
        let mut q = Queue::new();
        q.add(1);
        q.add(2);
        assert_eq!(1, q.remove().unwrap());
        assert_eq!(1, q.len());
    }
}
