use std::collections::BinaryHeap;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut consecutive_ones = vec![0; n];
        let mut ret = 0;

        for r in 0..m {
            let mut heap = BinaryHeap::new();

            for c in 0..n {
                consecutive_ones[c] = (consecutive_ones[c] + 1) * matrix[r][c];
                if consecutive_ones[c] > 0 {
                    heap.push(consecutive_ones[c]);
                }
            }

            for length in 1..=heap.len() as i32 {
                ret = ret.max(length * heap.pop().unwrap());
            }
        }

        ret
    }
}
