# 1768. 交替合并字符串
给你两个字符串 `word1` 和 `word2` 。请你从 `word1` 开始，通过交替添加字母来合并字符串。如果一个字符串比另一个字符串长，就将多出来的字母追加到合并后字符串的末尾。

返回 **合并后的字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> word1 = "abc", word2 = "pqr"
<strong>输出:</strong> "apbqcr"
<strong>解释:</strong> 字符串合并情况如下所示：
word1：  a   b   c
word2：    p   q   r
合并后：  a p b q c r
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> word1 = "ab", word2 = "pqrs"
<strong>输出:</strong> "apbqrs"
<strong>解释:</strong> 注意，word2 比 word1 长，"rs" 需要追加到合并后字符串的末尾。
word1：  a   b
word2：    p   q   r   s
合并后：  a p b q   r   s
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> word1 = "abcd", word2 = "pq"
<strong>输出:</strong> "apbqcd"
<strong>解释:</strong> 注意，word1 比 word2 长，"cd" 需要追加到合并后字符串的末尾。
word1：  a   b   c   d
word2：    p   q
合并后：  a p b q c   d
</pre>

#### 提示:
* `1 <= word1.length, word2.length <= 100`
* `word1` 和 `word2` 由小写英文字母组成

## 题解 (Rust)

### 1. 递归
```Rust
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        if word1.is_empty() && word2.is_empty() {
            return String::new();
        }

        format!(
            "{}{}{}",
            word1.get(..1).unwrap_or(""),
            word2.get(..1).unwrap_or(""),
            Self::merge_alternately(
                word1.get(1..).unwrap_or("").to_string(),
                word2.get(1..).unwrap_or("").to_string()
            )
        )
    }
}
```
