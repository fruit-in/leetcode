# 1400. 构造 K 个回文字符串
给你一个字符串 `s` 和一个整数 `k` 。请你用 s 字符串中 **所有字符** 构造 `k` 个非空 **回文串** 。

如果你可以用 `s` 中所有字符构造 `k` 个回文字符串，那么请你返回 **True** ，否则返回 **False** 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "annabelle", k = 2
<strong>输出:</strong> true
<strong>解释:</strong> 可以用 s 中所有字符构造 2 个回文字符串。
一些可行的构造方案包括："anna" + "elble"，"anbna" + "elle"，"anellena" + "b"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "leetcode", k = 3
<strong>输出:</strong> false
<strong>解释:</strong> 无法用 s 中所有字符构造 3 个回文串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "true", k = 4
<strong>输出:</strong> true
<strong>解释:</strong> 唯一可行的方案是让 s 中每个字符单独构成一个字符串。
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "yzyzyzyzyzyzyzy", k = 2
<strong>输出:</strong> true
<strong>解释:</strong> 你只需要将所有的 z 放在一个字符串中，所有的 y 放在另一个字符串中。那么两个字符串都是回文串。
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "cr", k = 7
<strong>输出:</strong> false
<strong>解释:</strong> 我们没有足够的字符去构造 7 个回文串。
</pre>

#### 提示:
* `1 <= s.length <= 10^5`
* `s` 中所有字符都是小写英文字母。
* `1 <= k <= 10^5`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let count = s.bytes().fold(0i32, |acc, x| acc ^ (1 << (x - b'a')));

        count.count_ones() as i32 <= k && s.len() as i32 >= k
    }
}
```
