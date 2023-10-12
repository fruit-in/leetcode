impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = vec![-10001];

        for &num in &nums {
            if num > *stack.last().unwrap() {
                stack.push(num);
            } else if let Err(i) = stack.binary_search(&num) {
                stack[i] = num;
            }
        }

        stack.len() as i32 - 1
    }
}
