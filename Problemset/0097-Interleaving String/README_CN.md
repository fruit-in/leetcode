# 97. 交错字符串
给定三个字符串 `s1`、`s2`、`s3`，请你帮忙验证 `s3` 是否是由 `s1` 和 `s2` **交错** 组成的。

两个字符串 `s` 和 `t` **交错** 的定义与过程如下，其中每个字符串都会被分割成若干 **非空** 子字符串：
* <code>s = s<sub>1</sub> + s<sub>2</sub> + ... + s<sub>n</sub></code>
* <code>t = t<sub>1</sub> + t<sub>2</sub> + ... + t<sub>m</sub></code>
* `|n - m| <= 1`
* **交错** 是 <code>s<sub>1</sub> + t<sub>1</sub> + s<sub>2</sub> + t<sub>2</sub> + s<sub>3</sub> + t<sub>3</sub> + ...</code> 或者 <code>t<sub>1</sub> + s<sub>1</sub> + t<sub>2</sub> + s<sub>2</sub> + t<sub>3</sub> + s<sub>3</sub> + ...</code>

**注意：**`a + b` 意味着字符串 `a` 和 `b` 连接。

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/02/interleave.jpg)
<pre>
<strong>输入:</strong> s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "", s2 = "", s3 = ""
<strong>输出:</strong> true
</pre>

#### 提示:
* `0 <= s1.length, s2.length <= 100`
* `0 <= s3.length <= 200`
* `s1`、`s2`、和 `s3` 都由小写英文字母组成

**进阶：**您能否仅使用 `O(s2.length)` 额外的内存空间来解决它?

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let mut dp = vec![false; s2.len() + 1];

        for i in 0..=s1.len() {
            let mut tmp = vec![false; s2.len() + 1];
            tmp[0] = i == 0;

            for j in 0..=s2.len() {
                tmp[j] |= i > 0 && s1[i - 1] == s3[i + j - 1] && dp[j];
                tmp[j] |= j > 0 && s2[j - 1] == s3[i + j - 1] && tmp[j - 1];
            }

            dp = tmp;
        }

        dp[s2.len()]
    }
}
```
