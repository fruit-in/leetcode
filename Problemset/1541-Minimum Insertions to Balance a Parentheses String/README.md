# 1541. Minimum Insertions to Balance a Parentheses String
Given a parentheses string `s` containing only the characters `'('` and `')'`. A parentheses string is **balanced** if:
* Any left parenthesis `'('` must have a corresponding two consecutive right parenthesis `'))'`.
* Left parenthesis `'('` must go before the corresponding two consecutive right parenthesis `'))'`.

In other words, we treat `'('` as an opening parenthesis and `'))'` as a closing parenthesis.

* For example, `"())"`, `"())(())))"` and `"(())())))"` are balanced, `")()"`, `"()))"` and `"(()))"` are not balanced.

You can insert the characters `'('` and `')'` at any position of the string to balance it if needed.

Return *the minimum number of insertions* needed to make `s` balanced.

#### Example 1:
<pre>
<strong>Input:</strong> s = "(()))"
<strong>Output:</strong> 1
<strong>Explanation:</strong> The second '(' has two matching '))', but the first '(' has only ')' matching. We need to add one more ')' at the end of the string to be "(())))" which is balanced.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "())"
<strong>Output:</strong> 0
<strong>Explanation:</strong> The string is already balanced.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "))())("
<strong>Output:</strong> 3
<strong>Explanation:</strong> Add '(' to match the first '))', Add '))' to match the last '('.
</pre>

#### Constraints:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` consists of `'('` and `')'` only.

## Solutions (Rust)

### 1. Solution
```Rust
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
```
