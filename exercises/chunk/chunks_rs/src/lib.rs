pub fn chunk(mut array: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    let mut chunks: Vec<Vec<i32>> = Vec::new();

    while array.len() > size {
        chunks.push(array.drain(0..size).collect());
    }
    chunks.push(array);

    chunks
}

#[cfg(test)]
mod chunks {
    use super::*;

    #[test]
    fn ten_size_2() {
        let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = chunk(v, 2);
        let solution =
            vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
        assert_eq!(solution, result);
    }

    #[test]
    fn three_size_1() {
        let v = vec![1, 2, 3];
        let result = chunk(v, 1);
        let solution = vec![vec![1], vec![2], vec![3]];

        assert_eq!(solution, result);
    }

    #[test]
    fn five_size_3() {
        let v = vec![1, 2, 3, 4, 5];
        let result = chunk(v, 3);
        let solution = vec![vec![1, 2, 3], vec![4, 5]];

        assert_eq!(solution, result);
    }

    #[test]
    fn thirteen_size_5() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let result = chunk(v, 5);
        let solution =
            vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10], vec![11, 12, 13]];

        assert_eq!(solution, result);
    }
}
