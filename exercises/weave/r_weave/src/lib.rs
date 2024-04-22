mod queue;
use queue::Queue;

pub fn weave<T: PartialEq + std::fmt::Debug>(
    mut q_one: Queue<T>,
    mut q_two: Queue<T>,
) -> Queue<T> {
}

#[cfg(test)]
mod weave_tests {
    use super::*;

    #[test]
    fn weaves() {
        let mut q_one: Queue<i32> = Queue::new();
        q_one.add(1);
        q_one.add(3);
        let mut q_two: Queue<i32> = Queue::new();
        q_two.add(2);
        q_two.add(4);
        q_two.add(5);
        let mut q_three = weave(q_one, q_two);

        let mut expect = Queue::new();
        expect.add(1);
        expect.add(2);
        expect.add(3);
        expect.add(4);
        expect.add(5);

        let mut v_result = Vec::new();
        for _ in 0..q_three.len() {
            v_result.push(q_three.remove());
        }
        let mut v_expect = Vec::new();
        for _ in 0..expect.len() {
            v_expect.push(expect.remove());
        }

        assert_eq!(v_result, v_expect);

        //        assert_eq!(q_three.len(), 5);
        //        assert_eq!(expect.remove(), q_three.remove());
        //        assert_eq!(expect.remove(), q_three.remove());
        //        assert_eq!(expect.remove(), q_three.remove());
        //        assert_eq!(expect.remove(), q_three.remove());
        //        assert_eq!(expect.remove(), q_three.remove());
    }
}
