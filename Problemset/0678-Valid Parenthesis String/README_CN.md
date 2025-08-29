# 678. 有效的括号字符串
给你一个只包含三种字符的字符串，支持的字符类型分别是 `'('`、`')'` 和 `'*'`。请你检验这个字符串是否为有效字符串，如果是 **有效** 字符串返回 `true` 。

**有效** 字符串符合如下规则：
* 任何左括号 `'('` 必须有相应的右括号 `')'`。
* 任何右括号 `')'` 必须有相应的左括号 `'('` 。
* 左括号 `'('` 必须在对应的右括号之前 `')'`。
* `'*'` 可以被视为单个右括号 `')'` ，或单个左括号 `'('` ，或一个空字符串 `""`。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "()"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "(*)"
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "(*))"
<strong>输出:</strong> true
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s[i]` 为 `'('`、`')'` 或 `'*'`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut possible_counts = HashSet::from([0]);

        for c in s.chars() {
            let mut next_counts = HashSet::new();

            for count in possible_counts.into_iter() {
                if c == '(' {
                    next_counts.insert(count + 1);
                } else if c == ')' && count > 0 {
                    next_counts.insert(count - 1);
                } else if c == '*' {
                    next_counts.insert(count);
                    next_counts.insert(count + 1);
                    next_counts.insert((count - 1).max(0));
                }
            }

            possible_counts = next_counts;
        }

        possible_counts.contains(&0)
    }
}
```
