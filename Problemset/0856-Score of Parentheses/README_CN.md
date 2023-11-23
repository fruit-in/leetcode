# 856. 括号的分数
给定一个平衡括号字符串 `S`，按下述规则计算该字符串的分数：

* `()` 得 `1` 分。
* `AB` 得 `A + B` 分，其中 `A` 和 `B` 是平衡括号字符串。
* `(A)` 得 `2 * A` 分，其中 `A` 是平衡括号字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> "()"
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "(())"
<strong>输出:</strong> 2
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "()()"
<strong>输出:</strong> 2
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> "(()(()))"
<strong>输出:</strong> 6
</pre>

#### 提示:
1. `S` 是平衡括号字符串，且只含有 `(` 和 `)` 。
2. `2 <= S.length <= 50`

## 题解 (Rust)

### 1. 题解
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
