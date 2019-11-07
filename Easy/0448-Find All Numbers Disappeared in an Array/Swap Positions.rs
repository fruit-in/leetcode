impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut ret = Vec::new();

        for i in 0..nums.len() {
            let mut curr = nums[i];

            while nums[curr as usize - 1] != curr {
                let next = nums[curr as usize - 1];
                nums[curr as usize - 1] = curr;
                curr = next;
            }
        }

        for i in 0..nums.len() {
            if i + 1 != nums[i] as usize {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
