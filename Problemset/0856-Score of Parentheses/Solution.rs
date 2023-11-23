impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![];

        for c in s.chars() {
            if c == '(' {
                stack.push(0);
            } else {
                match stack.pop().unwrap() {
                    0 => stack.push(1),
                    x => *stack.last_mut().unwrap() = 2 * x,
                }

                if stack.len() > 1 && stack[stack.len() - 2] > 0 {
                    let x = stack.pop().unwrap();
                    *stack.last_mut().unwrap() += x;
                }
            }
        }

        stack[0]
    }
}
