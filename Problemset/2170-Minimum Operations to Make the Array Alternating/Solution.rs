use std::collections::HashMap;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut even_count = HashMap::new();
        let mut odd_count = HashMap::new();
        let mut even_max2 = [(0, 0), (0, 0)];
        let mut odd_max2 = [(0, 0), (0, 0)];

        for i in 0..n as usize {
            if i % 2 == 0 {
                *even_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == even_max2[0].0 {
                    even_max2[0].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[0].1 {
                    even_max2[1] = even_max2[0];
                    even_max2[0] = (nums[i], even_count[&nums[i]]);
                } else if nums[i] == even_max2[1].0 {
                    even_max2[1].1 = even_count[&nums[i]];
                } else if even_count[&nums[i]] > even_max2[1].1 {
                    even_max2[1] = (nums[i], even_count[&nums[i]]);
                }
            } else {
                *odd_count.entry(nums[i]).or_insert(0) += 1;
                if nums[i] == odd_max2[0].0 {
                    odd_max2[0].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[0].1 {
                    odd_max2[1] = odd_max2[0];
                    odd_max2[0] = (nums[i], odd_count[&nums[i]]);
                } else if nums[i] == odd_max2[1].0 {
                    odd_max2[1].1 = odd_count[&nums[i]];
                } else if odd_count[&nums[i]] > odd_max2[1].1 {
                    odd_max2[1] = (nums[i], odd_count[&nums[i]]);
                }
            }
        }

        if even_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                n / 2 - odd_max2[1].1
            } else {
                n / 2 - odd_max2[0].1
            }
        } else if odd_max2[1].0 == 0 {
            if even_max2[0].0 == odd_max2[0].0 {
                (n + 1) / 2 - even_max2[1].1
            } else {
                (n + 1) / 2 - even_max2[0].1
            }
        } else {
            if even_max2[0].0 == odd_max2[0].0 {
                n - (even_max2[0].1 + odd_max2[1].1).max(even_max2[1].1 + odd_max2[0].1)
            } else {
                n - even_max2[0].1 - odd_max2[0].1
            }
        }
    }
}
