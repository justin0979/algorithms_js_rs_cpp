//
////
///
///
///
///
///
///
///
///
///
///
///
///
///
///
///
pub fn matrix(n: usize) -> Vec<Vec<usize>> {
    let mut m = vec![vec![0; n]; n];

    let mut count = 1;
    let mut forward = 0;
    let mut row_end = n - 1;
    let mut col_end = n - 1;
    let mut row_start = 0;
    let mut col_start = 0;

    while count <= n * n {
        if forward % 2 == 0 {
            for col in col_start..=col_end {
                m[row_start][col] = count;
                count += 1;
            }
            row_start += 1;

            for row in row_start..=row_end {
                m[row][col_end] = count;
                count += 1;
            }
            col_end -= 1;
        } else {
            for col in (col_start..=col_end).rev() {
                m[row_end][col] = count;
                count += 1;
            }
            row_end -= 1;

            for row in (row_start..=row_end).rev() {
                m[row][col_start] = count;
                count += 1;
            }
            col_start += 1;
        }
        forward += 1;
    }

    m
}

#[cfg(test)]
mod matrix_test {
    use r_matrix::matrix;

    #[test]
    fn two() {
        let expect = vec![vec![1, 2], vec![4, 3]];
        let result = matrix(2);
        assert_eq!(expect, result);
    }

    #[test]
    fn three() {
        let expect = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let result = matrix(3);
        assert_eq!(expect, result);
    }

    #[test]
    fn four() {
        let expect = vec![
            vec![1, 2, 3, 4],
            vec![12, 13, 14, 5],
            vec![11, 16, 15, 6],
            vec![10, 9, 8, 7],
        ];
        let result = matrix(4);
        assert_eq!(expect, result);
    }
}
