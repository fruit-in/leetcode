impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut ret = vec![];

        for j in 0..nums.len() {
            if nums[j] == key {
                for i in (j as i32 - k).max(0)..(j as i32 + k + 1).min(nums.len() as i32) {
                    if i > *ret.last().unwrap_or(&-1) {
                        ret.push(i);
                    }
                }
            }
        }

        ret
    }
}
