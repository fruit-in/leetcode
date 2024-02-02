impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        nums.push(0);

        for i in 0..nums.len() {
            while nums[i] >= 0 && nums[i] < nums.len() as i32 && nums[nums[i] as usize] != nums[i] {
                let tmp = nums[i] as usize;
                nums.swap(tmp, i);
            }
        }

        for i in 1..nums.len() {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }

        nums.len() as i32
    }
}
