# 2299. 强密码检验器 II
如果一个密码满足以下所有条件，我们称它是一个 **强** 密码：
* 它有至少 `8` 个字符。
* 至少包含 **一个小写英文** 字母。
* 至少包含 **一个大写英文** 字母。
* 至少包含 **一个数字** 。
* 至少包含 **一个特殊字符** 。特殊字符为：`"!@#$%^&*()-+"` 中的一个。
* 它 不 包含 `2` 个连续相同的字符（比方说 `"aab"` 不符合该条件，但是 `"aba"` 符合该条件）。

给你一个字符串 `password` ，如果它是一个 **强** 密码，返回 `true`，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> password = "IloveLe3tcode!"
<strong>输出:</strong> true
<strong>解释:</strong> 密码满足所有的要求，所以我们返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> password = "Me+You--IsMyDream"
<strong>输出:</strong> false
<strong>解释:</strong> 密码不包含数字，且包含 2 个连续相同的字符。所以我们返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> password = "1aB!"
<strong>输出:</strong> false
<strong>解释:</strong> 密码不符合长度要求。所以我们返回 false 。
</pre>

#### 提示:
* `1 <= password.length <= 100`
* `password` 包含字母，数字和 `"!@#$%^&*()-+"` 这些特殊字符。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        password.len() >= 8
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_digit(10))
            && password.chars().any(|c| "!@#$%^&*()-+".contains(c))
            && password
                .chars()
                .zip(password.chars().skip(1))
                .all(|(c1, c2)| c1 != c2)
    }
}
```
