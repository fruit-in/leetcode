# 1433. 检查一个字符串是否可以打破另一个字符串
给你两个字符串 `s1` 和 `s2` ，它们长度相等，请你检查是否存在一个 `s1`  的排列可以打破 `s2` 的一个排列，或者是否存在一个 `s2` 的排列可以打破 `s1` 的一个排列。

字符串 `x` 可以打破字符串 `y` （两者长度都为 `n` ）需满足对于所有 `i`（在 `0` 到 `n - 1` 之间）都有 `x[i] >= y[i]`（字典序意义下的顺序）。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "abc", s2 = "xya"
<strong>输出:</strong> true
<strong>解释:</strong> "ayx" 是 s2="xya" 的一个排列，"abc" 是字符串 s1="abc" 的一个排列，且 "ayx" 可以打破 "abc" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "abe", s2 = "acd"
<strong>输出:</strong> false
<strong>解释:</strong> s1="abe" 的所有排列包括："abe"，"aeb"，"bae"，"bea"，"eab" 和 "eba" ，s2="acd" 的所有排列包括："acd"，"adc"，"cad"，"cda"，"dac" 和 "dca"。然而没有任何 s1 的排列可以打破 s2 的排列。也没有 s2 的排列能打破 s1 的排列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "leetcodee", s2 = "interview"
<strong>输出:</strong> true
</pre>

#### 提示:
* `s1.length == n`
* `s2.length == n`
* `1 <= n <= 10^5`
* 所有字符串都只包含小写英文字母。

## 题解 (Rust)

### 1. 排序
```Rust
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort_unstable();
        s2.sort_unstable();
        let s1s2 = s1.iter().zip(s2.iter()).collect::<Vec<_>>();

        s1s2.iter().all(|tup| tup.0 >= tup.1) || s1s2.iter().all(|tup| tup.0 <= tup.1)
    }
}
```
