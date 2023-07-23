impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(2 * n as usize);

        for i in 0..(n as usize) {
            ret.push(nums[i]);
            ret.push(nums[n as usize + i]);
        }

        ret
    }
}
