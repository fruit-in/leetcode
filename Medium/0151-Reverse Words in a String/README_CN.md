# 151. 翻转字符串里的单词
给定一个字符串，逐个翻转字符串中的每个单词。

#### 示例 1:
<pre>
<strong>输入:</strong> "the sky is blue"
<strong>输出:</strong> "blue is sky the"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "  hello world!  "
<strong>输出:</strong> "world! hello"
<strong>解释:</strong> 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "a good   example"
<strong>输出:</strong> "example good a"
<strong>解释:</strong> 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。
</pre>

#### 说明:
* 无空格字符构成一个单词。
* 输入字符串可以在前面或者后面包含多余的空格，但是反转后的字符不能包括。
* 如果两个单词间有多余的空格，将反转后单词间的空格减少到只含一个。

#### 进阶:
请选用 C 语言的用户尝试使用 *O*(1) 额外空间复杂度的原地解法。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
    }
}
```
