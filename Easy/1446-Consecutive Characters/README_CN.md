# 1446. 连续字符
给你一个字符串 `s` ，字符串的「能量」定义为：只包含一种字符的最长非空子字符串的长度。

请你返回字符串的能量。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leetcode"
<strong>输出:</strong> 2
<strong>解释:</strong> 子字符串 "ee" 长度为 2 ，只包含字符 'e' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abbcccddddeeeeedcba"
<strong>输出:</strong> 5
<strong>解释:</strong> 子字符串 "eeeee" 长度为 5 ，只包含字符 'e' 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "triplepillooooow"
<strong>输出:</strong> 5
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "hooraaaaaaaaaaay"
<strong>输出:</strong> 11
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> s = "tourist"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= s.length <= 500`
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut s = s.bytes().peekable();
        let mut count = 1;
        let mut power = 1;

        while let Some(ch0) = s.next() {
            if let Some(&ch1) = s.peek() {
                if ch1 == ch0 {
                    count += 1;
                } else {
                    power = power.max(count);
                    count = 1;
                }
            }
        }

        power.max(count)
    }
}
```
