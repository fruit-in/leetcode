impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut stack = vec![(0, nums.len() - 1)];
        let mut ret = nums[0];

        while let Some((l, r)) = stack.pop() {
            if l == r || nums[l] < nums[r] {
                ret = ret.min(nums[l]);
                continue;
            }

            let mid = (l + r) / 2;

            if nums[mid] > nums[l] || (nums[l] != nums[r] && nums[mid] == nums[l]) {
                stack.push((mid + 1, r));
            } else if nums[mid] < nums[r] || (nums[l] != nums[r] && nums[mid] == nums[r]) {
                stack.push((l, mid));
            } else {
                stack.push((mid + 1, r));
                stack.push((l, mid));
            }
        }

        ret
    }
}
