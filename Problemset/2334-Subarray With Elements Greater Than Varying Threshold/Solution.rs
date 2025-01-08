impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut sublens = vec![0; nums.len()];
        let mut asc_stack = vec![];

        for i in 0..nums.len() {
            while asc_stack.last().unwrap_or(&(-1, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] = i as i32 - asc_stack.last().unwrap_or(&(-1, 0)).0;
            asc_stack.push((i as i32, nums[i]));
        }

        asc_stack.clear();

        for i in (0..nums.len()).rev() {
            while asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] += asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).0 - i as i32 - 1;
            asc_stack.push((i as i32, nums[i]));

            if nums[i] > threshold / sublens[i] {
                return sublens[i];
            }
        }

        -1
    }
}
