# 1745. 分割回文串 IV
给你一个字符串 `s` ，如果可以将它分割成三个 **非空** 回文子字符串，那么返回 `true` ，否则返回 `false` 。

当一个字符串正着读和反着读是一模一样的，就称其为 **回文字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcbdd"
<strong>输出:</strong> true
<strong>解释:</strong> "abcbdd" = "a" + "bcb" + "dd"，三个子字符串都是回文的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "bcbddxy"
<strong>输出:</strong> false
<strong>解释:</strong> s 没办法被分割成 3 个回文子字符串。
</pre>

#### 提示:
* `3 <= s.length <= 2000`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.as_bytes();
        let mut is_palindrome = vec![vec![false; s.len()]; s.len()];

        for r in 0..s.len() {
            for l in 0..=r {
                is_palindrome[l][r] = s[l] == s[r];
                if is_palindrome[l][r] && l + 2 < r {
                    is_palindrome[l][r] = is_palindrome[l + 1][r - 1];
                }
            }
        }

        for l in 1..s.len() - 1 {
            for r in l..s.len() - 1 {
                if is_palindrome[0][l - 1]
                    && is_palindrome[l][r]
                    && is_palindrome[r + 1][s.len() - 1]
                {
                    return true;
                }
            }
        }

        false
    }
}
```
