# 2278. 字母在字符串中的百分比
给你一个字符串 `s` 和一个字符 `letter` ，返回在 `s` 中等于 `letter` 字符所占的 **百分比** ，向下取整到最接近的百分比。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "foobar", letter = "o"
<strong>输出:</strong> 33
<strong>解释:</strong>
等于字母 'o' 的字符在 s 中占到的百分比是 2 / 6 * 100% = 33% ，向下取整，所以返回 33 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "jjjj", letter = "k"
<strong>输出:</strong> 0
<strong>解释:</strong>
等于字母 'k' 的字符在 s 中占到的百分比是 0% ，所以返回 0 。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `s` 由小写英文字母组成
* `letter` 是一个小写英文字母

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        (s.chars().filter(|&c| c == letter).count() * 100 / s.len()) as i32
    }
}
```
