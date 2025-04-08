impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let mut prefix_sum = 0;
        let mut stack = vec![(0, -1)];
        let mut ret = i32::MAX;

        for i in 0..nums.len() {
            prefix_sum += nums[i] as i64;

            while stack.last().unwrap_or(&(i64::MIN, 0)).0 >= prefix_sum {
                stack.pop();
            }
            stack.push((prefix_sum, i as i32));

            let j = stack
                .binary_search(&(prefix_sum - k, i as i32))
                .unwrap_err();
            if j > 0 {
                ret = ret.min(i as i32 - stack[j - 1].1);
            }
        }

        if ret == i32::MAX {
            return -1;
        }

        ret
    }
}
