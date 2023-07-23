impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut v = Vec::new();
        for i in 0..nums.len() {
            v.push((nums[i], i));
        }
        v.sort_unstable();
        for i in 1..v.len() {
            if v[i - 1].0 == v[i].0 && v[i].1 <= v[i - 1].1 + k {
                return true;
            }
        }
        false
    }
}
