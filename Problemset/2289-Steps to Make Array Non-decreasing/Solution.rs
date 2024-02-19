impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut ret = 0;

        for &num in &nums {
            let mut x = 0;

            while stack.last().unwrap_or(&(i32::MAX, 0)).0 <= num {
                x = x.max(stack.pop().unwrap().1);
            }

            if stack.is_empty() {
                stack.push((num, 0));
            } else {
                stack.push((num, x + 1));
                ret = ret.max(x + 1);
            }
        }

        ret
    }
}
