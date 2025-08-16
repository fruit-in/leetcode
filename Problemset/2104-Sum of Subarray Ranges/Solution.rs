impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count_as_max = vec![0; n];
        let mut count_as_min = vec![0; n];

        let mut dec_stack = vec![];
        let mut inc_stack = vec![];
        for i in 0..n {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 < nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 > nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] = i as i64 - dec_stack.last().unwrap_or(&(-1, 0)).0;
            count_as_min[i] = i as i64 - inc_stack.last().unwrap_or(&(-1, 0)).0;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        dec_stack = vec![];
        inc_stack = vec![];
        for i in (0..n).rev() {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 <= nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 >= nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] *= dec_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            count_as_min[i] *= inc_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        (0..n)
            .map(|i| (count_as_max[i] - count_as_min[i]) * nums[i] as i64)
            .sum()
    }
}
