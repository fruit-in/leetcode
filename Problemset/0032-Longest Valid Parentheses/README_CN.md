# 32. 最长有效括号
给你一个只包含 `'('` 和 `')'` 的字符串，找出最长有效（格式正确且连续）括号子串的长度。

左右括号匹配，即每个左括号都有对应的右括号将其闭合的字符串是格式正确的，比如 `"(()())"`。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "(()"
<strong>输出:</strong> 2
<strong>解释:</strong> 最长有效括号子串是 "()"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = ")()())"
<strong>输出:</strong> 4
<strong>解释:</strong> 最长有效括号子串是 "()()"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = ""
<strong>输出:</strong> 0
</pre>

#### 提示:
* <code>0 <= s.length <= 3 * 10<sup>4</sup></code>
* `s[i]` 为 `'('` 或 `')'`

## 题解 (Rust)

### 1. 题解
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
