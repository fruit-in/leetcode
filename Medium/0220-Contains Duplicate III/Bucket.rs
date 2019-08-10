use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let mut buckets = HashMap::new();
        for i in 0..nums.len() {
            let bucket_id = (nums[i] as f64 / (t + 1) as f64).floor() as i64;
            if buckets.insert(bucket_id, nums[i]) != None {
                return true;
            }
            match buckets.get(&(bucket_id - 1)) {
                Some(n) => {
                    if (n - nums[i]).abs() <= t {
                        return true;
                    }
                },
                None => (),
            }
            match buckets.get(&(bucket_id + 1)) {
                Some(n) => {
                    if (n - nums[i]).abs() <= t {
                        return true;
                    }
                },
                None => (),
            }
            if i >= k {
                buckets.remove(&(nums[i - k] / (t + 1)));
            }
        }
        false
    }
}
