impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut ret = vec![-1, -1];

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if nums[left] != target {
            return vec![-1, -1];
        }

        ret[0] = left as i32;
        left = 0;
        right = n - 1;

        while left < right {
            let mid = (left + right + 1) / 2;

            if nums[mid] <= target {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        ret[1] = left as i32;

        ret
    }
}
