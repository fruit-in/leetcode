# 58. 最后一个单词的长度
给定一个仅包含大小写字母和空格 ```' '``` 的字符串，返回其最后一个单词的长度。

如果不存在最后一个单词，请返回 0 。

**说明:** 一个单词是指由字母组成，但不包含任何空格的字符串。

#### 示例:
<pre>
<strong>输入:</strong> "Hello World"
<strong>输出:</strong> 5
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut cnt = 0;
        let mut flag = false;
        for c in s.chars() {
            if c == ' ' {
                flag = true;
            } else if flag {
                cnt = 1;
                flag = false;
            } else {
                cnt += 1;
            }
        }
        cnt
    }
}
```
