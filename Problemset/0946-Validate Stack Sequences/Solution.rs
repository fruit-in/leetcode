impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut i = 0;

        for val in pushed {
            stack.push(val);
            while i < popped.len() && *stack.last().unwrap_or(&-1) == popped[i] {
                stack.pop();
                i += 1;
            }
        }

        stack.is_empty()
    }
}
