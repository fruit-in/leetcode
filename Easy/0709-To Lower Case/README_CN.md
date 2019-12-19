# 709. 转换成小写字母
实现函数 ToLowerCase()，该函数接收一个字符串参数 str，并将该字符串中的大写字母转换成小写字母，之后返回新的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> "Hello"
<strong>输出:</strong> "hello"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "here"
<strong>输出:</strong> "here"
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> "LOVELY"
<strong>输出:</strong> "lovely"
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut lower_case_str = String::new();
        for ch in str.chars() {
            lower_case_str.push(ch.to_ascii_lowercase());
        }
        lower_case_str
    }
}
```
