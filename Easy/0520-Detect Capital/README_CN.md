# 520. 检测大写字母
给定一个单词，你需要判断单词的大写使用是否正确。

我们定义，在以下情况时，单词的大写用法是正确的：
1. 全部字母都是大写，比如"USA"。
2. 单词中所有字母都不是大写，比如"leetcode"。
3. 如果单词不只含有一个字母，只有首字母大写， 比如 "Google"。

否则，我们定义这个单词没有正确使用大写字母。

#### 示例 1:
<pre>
<strong>输入:</strong> "USA"
<strong>输出:</strong> True
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> "FlaG"
<strong>输出:</strong> False
</pre>

**注意:** 输入是由大写和小写拉丁字母组成的非空单词。

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut chars = word.chars();
        if let Some(ch) = chars.next() {
            if ch.is_ascii_uppercase() {
                if let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_lowercase() {
                                return false;
                            }
                        }
                    } else {
                        while let Some(ch) = chars.next() {
                            if ch.is_ascii_uppercase() {
                                return false;
                            }
                        }
                    }
                }
            } else {
                while let Some(ch) = chars.next() {
                    if ch.is_ascii_uppercase() {
                        return false;
                    }
                }
            }
        }
        true
    }
}
```
