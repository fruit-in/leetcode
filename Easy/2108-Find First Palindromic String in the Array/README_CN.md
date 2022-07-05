# 2108. 找出数组中的第一个回文字符串
给你一个字符串数组 `words` ，找出并返回数组中的 **第一个回文字符串** 。如果不存在满足要求的字符串，返回一个 **空字符串** `""` 。

**回文字符串** 的定义为：如果一个字符串正着读和反着读一样，那么该字符串就是一个 **回文字符串** 。

#### 示例 1:
<pre>
<strong>输入:</strong> words = ["abc","car","ada","racecar","cool"]
<strong>输出:</strong> "ada"
<strong>解释:</strong> 第一个回文字符串是 "ada" 。
注意，"racecar" 也是回文字符串，但它不是第一个。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> words = ["notapalindrome","racecar"]
<strong>输出:</strong> "racecar"
<strong>解释:</strong> 第一个也是唯一一个回文字符串是 "racecar" 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> words = ["def","ghi"]
<strong>输出:</strong> ""
<strong>解释:</strong> 不存在回文字符串，所以返回一个空字符串。
</pre>

#### 提示:
* `1 <= words.length <= 100`
* `1 <= words[i].length <= 100`
* `words[i]` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        words
            .into_iter()
            .find(|w| {
                let v = w.as_bytes();
                (0..v.len() / 2).all(|i| v[i] == v[v.len() - 1 - i])
            })
            .unwrap_or("".to_string())
    }
}
```
