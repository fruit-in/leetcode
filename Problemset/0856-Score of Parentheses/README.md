# 856. Score of Parentheses
Given a balanced parentheses string `s`, return *the **score** of the string*.

The **score** of a balanced parentheses string is based on the following rule:

* `"()"` has score `1`.
* `AB` has score `A + B`, where `A` and `B` are balanced parentheses strings.
* `(A)` has score `2 * A`, where `A` is a balanced parentheses string.

#### Example 1:
<pre>
<strong>Input:</strong> s = "()"
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "(())"
<strong>Output:</strong> 2
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "()()"
<strong>Output:</strong> 2
</pre>

#### Constraints:
* `2 <= s.length <= 50`
* `s` consists of only `'('` and `')'`.
* `s` is a balanced parentheses string.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
