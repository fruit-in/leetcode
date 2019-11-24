impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut min = std::i32::MAX;
        let mut max = std::i32::MIN;

        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                min = *nums.iter().skip(i).min().unwrap();
                break;
            }
        }

        for i in (0..(nums.len() - 1)).rev() {
            if nums[i] > nums[i + 1] {
                max = *nums.iter().take(i + 1).max().unwrap();
                break;
            }
        }

        let l = nums.iter().position(|&x| x > min).unwrap_or(nums.len());
        let r = nums.iter().rposition(|&x| x < max).unwrap_or(0);

        (r as i32 - l as i32 + 1).max(0)
    }
}
