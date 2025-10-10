# 32. Longest Valid Parentheses
Given a string containing just the characters `'('` and `')'`, return *the length of the longest valid (well-formed) parentheses substring*.

#### Example 1:
<pre>
<strong>Input:</strong> s = "(()"
<strong>Output:</strong> 2
<strong>Explanation:</strong> The longest valid parentheses substring is "()".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = ")()())"
<strong>Output:</strong> 4
<strong>Explanation:</strong> The longest valid parentheses substring is "()()".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = ""
<strong>Output:</strong> 0
</pre>

#### Constraints:
* <code>0 <= s.length <= 3 * 10<sup>4</sup></code>
* `s[i]` is `'('`, or `')'`.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
