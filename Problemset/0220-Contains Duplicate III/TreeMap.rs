use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let mut tree = BTreeSet::new();
        for i in 0..nums.len() {
            let n = nums[i];
            if tree.range((n - t)..=(n + t)).count() > 0 {
                return true;
            }
            tree.insert(n);
            if i >= k {
                tree.remove(&nums[i - k]);
            }
        }
        false
    }
}
