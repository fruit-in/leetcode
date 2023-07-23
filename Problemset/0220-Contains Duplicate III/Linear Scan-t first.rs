impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut len = nums.len();
        let mut v = Vec::new();
        for i in 0..len {
            v.push((nums[i] as i64, i as i32));
        }
        v.sort_unstable();
        for i in 0..len {
            let mut j = i + 1;
            while j < len && v[j].0 - v[i].0 <= t as i64 {
                if (v[j].1 - v[i].1).abs() <= k {
                    return true;
                }
                j += 1;
            }
        }
        false
    }
}
