use std::collections::HashMap;

impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64 {
        let n = nums1.len();
        let nums = (0..n).map(|i| nums1[i] - nums2[i]).collect::<Vec<_>>();
        let mut nums_diff = nums.clone();
        let mut hashmap = HashMap::new();
        let mut tree = vec![0; n * 2 + 1];
        let mut ret = 0;

        for i in 0..n {
            nums_diff.push(nums[i] + diff);
        }
        nums_diff.sort_unstable();
        for i in 1..tree.len() {
            hashmap.insert(nums_diff[i - 1], i as i32);
        }

        for i in 0..n {
            let mut temp = hashmap[&(nums[i] + diff)];

            while temp > 0 {
                ret += tree[temp as usize];
                temp -= temp & (-temp);
            }

            temp = hashmap[&nums[i]];

            while temp < tree.len() as i32 {
                tree[temp as usize] += 1;
                temp += temp & (-temp);
            }
        }

        ret
    }
}
