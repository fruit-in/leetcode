# 1653. 使字符串平衡的最少删除次数
给你一个字符串 `s` ，它仅包含字符 `'a'` 和 `'b'` 。

你可以删除 `s` 中任意数目的字符，使得 `s` **平衡** 。当不存在下标对 `(i,j)` 满足 `i < j` ，且 `s[i] = 'b'` 的同时 `s[j]= 'a'` ，此时认为 `s` 是 **平衡** 的。

请你返回使 `s` **平衡** 的 **最少** 删除次数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aababbab"
<strong>输出:</strong> 2
<strong>解释:</strong> 你可以选择以下任意一种方案：
下标从 0 开始，删除第 2 和第 6 个字符（"aababbab" -> "aaabbb"），
下标从 0 开始，删除第 3 和第 6 个字符（"aababbab" -> "aabbbb"）。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "bbaaaaabb"
<strong>输出:</strong> 2
<strong>解释:</strong> 唯一的最优解是删除最前面两个字符。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 要么是 `'a'` 要么是 `'b'` 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut count = s.chars().filter(|&c| c == 'a').count() as i32;
        let mut ret = count;

        for c in s.chars() {
            count += (c == 'b') as i32 - (c == 'a') as i32;
            ret = ret.min(count);
        }

        ret
    }
}
```
