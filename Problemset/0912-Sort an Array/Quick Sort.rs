impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() < 2 {
            return nums;
        }
        let mut nums = nums;
        let mut i = 1;
        for j in 1..nums.len() {
            if nums[j] < nums[0] {
                nums.swap(i, j);
                i += 1;
            }
        }
        nums.swap(i - 1, 0);
        let mut ret = Self::sort_array(nums[..i].to_vec());
        ret.append(&mut Self::sort_array(nums[i..].to_vec()));
        ret
    }
}
