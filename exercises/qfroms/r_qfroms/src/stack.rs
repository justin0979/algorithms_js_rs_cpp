#[derive(Debug, PartialEq)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::<T>::new(),
        }
    }

    pub fn add(&mut self, record: T) {
        self.data.push(record);
    }

    pub fn remove(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn s_peek(&self) -> Option<&T> {
        if self.data.len() > 0 {
            Some(&self.data[self.data.len() - 1])
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod stack_tests {
    use super::*;

    fn initialize<T>() -> Stack<T> {
        Stack::new()
    }

    #[test]
    fn adds_and_removes() {
        let mut q: Stack<i32> = initialize();
        q.add(1);
        q.add(2);
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.remove(), Some(1));
        assert_eq!(q.remove(), None);
    }

    #[test]
    fn s_peeks() {
        let mut q: Stack<i32> = initialize();
        q.add(1);
        q.add(2);
        assert_eq!(q.s_peek(), Some(&2));
        q.remove();
        assert_eq!(q.s_peek(), Some(&1));
        q.remove();
        assert_eq!(q.s_peek(), None);
    }
}
