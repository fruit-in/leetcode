use std::collections::BinaryHeap;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut nums = nums.iter().map(|&x| x as i64).collect::<BinaryHeap<_>>();
        let mut ret = 0;

        for _ in 0..k {
            let x = nums.pop().unwrap();
            nums.push((x + 2) / 3);
            ret += x;
        }

        ret
    }
}
