impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt = [0; 101];
        let mut ret = 0;

        for num in nums {
            ret += cnt[num as usize];
            cnt[num as usize] += 1;
        }

        ret
    }
}
