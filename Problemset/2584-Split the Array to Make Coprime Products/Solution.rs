use std::collections::HashMap;

impl Solution {
    pub fn find_valid_split(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut range = HashMap::new();
        let mut arr = vec![0; n + 1];
        let mut prefix_sum = 0;

        for i in 0..n {
            for x in 1..=nums[i].isqrt() {
                if nums[i] % x != 0 {
                    continue;
                }

                for y in [x, nums[i] / x] {
                    if !range.contains_key(&y) {
                        range.insert(y, [i, i]);
                    }
                    range.get_mut(&y).unwrap()[1] = i;
                }
            }
        }

        for (&k, &v) in range.iter() {
            if k == 1 || v[0] == v[1] {
                continue;
            }

            arr[v[0]] += 1;
            arr[v[1]] -= 1;
        }

        for i in 0..n - 1 {
            prefix_sum += arr[i];

            if prefix_sum == 0 {
                return i as i32;
            }
        }

        -1
    }
}
