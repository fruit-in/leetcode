impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut ret = vec![false; l.len()];

        for i in 0..l.len() {
            let mut sub = nums[l[i] as usize..=r[i] as usize].to_vec();
            sub.sort_unstable();
            ret[i] =
                sub.len() > 1 && (2..sub.len()).all(|j| sub[j] - sub[j - 1] == sub[1] - sub[0]);
        }

        ret
    }
}
