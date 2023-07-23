# 1869. 哪种连续子字符串更长
给你一个二进制字符串 `s` 。如果字符串中由 `1` 组成的 **最长** 连续子字符串 **严格长于** 由 `0` 组成的 **最长** 连续子字符串，返回 `true` ；否则，返回 `false` 。
* 例如，`s = "110100010"` 中，由 `1` 组成的最长连续子字符串的长度是 `2` ，由 `0` 组成的最长连续子字符串的长度是 `3` 。

注意，如果字符串中不存在 `0` ，此时认为由 `0` 组成的最长连续子字符串的长度是 `0` 。字符串中不存在 `1` 的情况也适用此规则。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "1101"
<strong>输出:</strong> true
<strong>解释:</strong>
由 1 组成的最长连续子字符串的长度是 2："1101"
由 0 组成的最长连续子字符串的长度是 1："1101"
由 1 组成的子字符串更长，故返回 true 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "111000"
<strong>输出:</strong> false
<strong>解释:</strong>
由 1 组成的最长连续子字符串的长度是 3："111000"
由 0 组成的最长连续子字符串的长度是 3："111000"
由 1 组成的子字符串不比由 0 组成的子字符串长，故返回 false 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "110100010"
<strong>输出:</strong> false
<strong>解释:</strong>
由 1 组成的最长连续子字符串的长度是 2："110100010"
由 0 组成的最长连续子字符串的长度是 3："110100010"
由 1 组成的子字符串不比由 0 组成的子字符串长，故返回 false 。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s[i]` 不是 `'0'` 就是 `'1'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut max0 = 0;
        let mut max1 = 0;
        let mut count = 0;
        let mut is0 = true;

        for c in s.chars() {
            match (c, is0) {
                ('0', true) | ('1', false) => count += 1,
                (_, true) => {
                    max0 = max0.max(count);
                    is0 = false;
                    count = 1;
                }
                (_, false) => {
                    max1 = max1.max(count);
                    is0 = true;
                    count = 1;
                }
            }
        }

        if is0 {
            max0 = max0.max(count);
        } else {
            max1 = max1.max(count);
        }

        max1 > max0
    }
}
```
