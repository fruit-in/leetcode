impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let t = t as i64;
        let len = nums.len();
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        for i in 0..len {
            for n in &nums[(i + 1)..len.min(i + 1 + k)] {
                if (nums[i] - n).abs() <= t {
                    return true;
                }
            }
        }
        false
    }
}
