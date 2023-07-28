impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let count_1 = nums.iter().filter(|&&x| x == 1).count();
        let mut count_0 = nums.iter().take(count_1).filter(|&&x| x == 0).count();
        let mut ret = count_0;

        for i in 0..nums.len() - 1 {
            count_0 -= (nums[i] == 0) as usize;
            count_0 += (nums[(i + count_1) % nums.len()] == 0) as usize;
            ret = ret.min(count_0);
        }

        ret as i32
    }
}
