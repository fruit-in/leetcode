use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from([0]);

        for i in 0..mat.len() {
            let mut tmp = BinaryHeap::new();

            while let Some(x) = heap.pop() {
                for j in 0..mat[0].len() {
                    tmp.push(x + mat[i][j]);

                    if tmp.len() as i32 > k {
                        tmp.pop();
                    }
                }
            }

            heap = tmp;
        }

        heap.pop().unwrap()
    }
}
