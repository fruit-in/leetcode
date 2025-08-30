impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut stack = vec![];
        let mut ret = 0;

        for c in s.chars() {
            if c == '(' && *stack.last().unwrap_or(&'(') == ')' {
                stack.pop();
                stack.push('(');
                ret += 1;
            } else if c == '(' {
                stack.push('(');
            } else if stack.is_empty() {
                stack.push(')');
                ret += 1;
            } else if *stack.last().unwrap() == '(' {
                stack.pop();
                stack.push(')');
            } else {
                stack.pop();
            }
        }

        if *stack.last().unwrap_or(&'(') == ')' {
            stack.pop();
            ret += 1;
        }
        ret += stack.len() as i32 * 2;

        ret
    }
}
