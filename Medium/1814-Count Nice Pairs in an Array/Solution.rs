use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for &num in &nums {
            count
                .entry(num - Self::rev(num))
                .and_modify(|x| *x += 1)
                .or_insert(1_i64);
        }

        (count.into_values().map(|x| (x - 1) * x / 2).sum::<i64>() % 1_000_000_007) as i32
    }

    pub fn rev(x: i32) -> i32 {
        let mut x = x;
        let mut ret = 0;

        while x > 0 {
            ret = ret * 10 + x % 10;
            x /= 10;
        }

        ret
    }
}
