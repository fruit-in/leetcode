impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        let mut ret = 0;

        for c in s.chars() {
            if c == '(' {
                stack.push(-1);
                stack.push(0);
            } else if stack.len() < 3 {
                stack = vec![0];
            } else {
                let x = stack.pop().unwrap() + 2;
                stack.pop();
                *stack.last_mut().unwrap() += x;
                ret = ret.max(*stack.last().unwrap());
            }
        }

        ret
    }
}
