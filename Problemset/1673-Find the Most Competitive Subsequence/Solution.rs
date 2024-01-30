impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut ret = vec![];

        for i in 0..nums.len() {
            while let Some(&num) = ret.last() {
                if num > nums[i] && k - ret.len() < nums.len() - i {
                    ret.pop();
                } else {
                    break;
                }
            }

            if ret.len() < k {
                ret.push(nums[i]);
            }
        }

        ret
    }
}
