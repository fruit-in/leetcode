# 1790. 仅执行一次字符串交换能否使两个字符串相等
给你长度相等的两个字符串 `s1` 和 `s2` 。一次 **字符串交换** 操作的步骤如下：选出某个字符串中的两个下标（不必不同），并交换这两个下标所对应的字符。

如果对 **其中一个字符串** 执行 **最多一次字符串交换** 就可以使两个字符串相等，返回 `true` ；否则，返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s1 = "bank", s2 = "kanb"
<strong>输出:</strong> true
<strong>解释:</strong> 例如，交换 s2 中的第一个和最后一个字符可以得到 "bank"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s1 = "attack", s2 = "defend"
<strong>输出:</strong> false
<strong>解释:</strong> 一次字符串交换无法使两个字符串相等
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s1 = "kelb", s2 = "kelb"
<strong>输出:</strong> true
<strong>解释:</strong> 两个字符串已经相等，所以不需要进行字符串交换
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s1 = "abcd", s2 = "dcba"
<strong>输出:</strong> false
</pre>

#### 提示:
* `1 <= s1.length, s2.length <= 100`
* `s1.length == s2.length`
* `s1` 和 `s2` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff = s1
            .chars()
            .zip(s2.chars())
            .filter(|(c0, c1)| c0 != c1)
            .take(3)
            .collect::<Vec<_>>();

        diff.is_empty() || (diff.len() == 2 && diff[0].0 == diff[1].1 && diff[0].1 == diff[1].0)
    }
}
```
