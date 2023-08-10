use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut heap = nums
            .iter()
            .enumerate()
            .map(|(i, x)| Reverse((x, i)))
            .collect::<BinaryHeap<_>>();
        let mut indices = HashSet::new();
        let mut score = 0;

        while indices.len() < nums.len() {
            let Reverse((x, i)) = heap.pop().unwrap();

            if !indices.contains(&i) {
                score += *x as i64;
                indices.insert(i);
                if i > 0 {
                    indices.insert(i - 1);
                }
                if i < nums.len() - 1 {
                    indices.insert(i + 1);
                }
            }
        }

        score
    }
}
