# 1156. 单字符重复子串的最大长度
如果字符串中的所有字符都相同，那么这个字符串是单字符重复的字符串。

给你一个字符串 `text`，你只能交换其中两个字符一次或者什么都不做，然后得到一些单字符重复的子串。返回其中最长的子串的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "ababa"
<strong>输出:</strong> 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "aaabaaa"
<strong>输出:</strong> 6
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text = "aaabbaaa"
<strong>输出:</strong> 4
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> text = "aaaaa"
<strong>输出:</strong> 5
</pre>

#### 示例 5:
<pre>
<strong>输入:</strong> text = "abcdef"
<strong>输出:</strong> 1
</pre>

#### 提示:
* <code>1 <= text.length <= 2 * 10<sup>4</sup></code>
* `text` 仅由小写英文字母组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        let text = text.as_bytes();
        let mut ranges = vec![vec![]; 26];
        let mut ret = 1;

        for i in 0..text.len() {
            if i == 0 || text[i] != text[i - 1] {
                ranges[(text[i] - b'a') as usize].push((i, i));
            } else {
                ranges[(text[i] - b'a') as usize].last_mut().unwrap().1 = i;
            }
        }

        for i in 0..ranges.len() {
            for j in 0..ranges[i].len() {
                ret = ret.max(ranges[i][j].1 - ranges[i][j].0 + 1 + (ranges[i].len() > 1) as usize);

                if j > 0 && ranges[i][j].0 == ranges[i][j - 1].1 + 2 {
                    ret = ret
                        .max(ranges[i][j].1 - ranges[i][j - 1].0 + (ranges[i].len() > 2) as usize);
                }
            }
        }

        ret as i32
    }
}
```
