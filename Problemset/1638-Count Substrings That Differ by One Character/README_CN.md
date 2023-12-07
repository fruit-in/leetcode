# 1638. 统计只差一个字符的子串数目
给你两个字符串 `s` 和 `t` ，请你找出 `s` 中的非空子串的数目，这些子串满足替换 **一个不同字符** 以后，是 `t` 串的子串。换言之，请你找到 `s` 和 `t` 串中 **恰好** 只有一个字符不同的子字符串对的数目。

比方说， <code>"<u>compute</u>r"</code> and <code>"<u>computa</u>tion"</code> 只有一个字符不同： `'e'`/`'a'` ，所以这一对子字符串会给答案加 1 。

请你返回满足上述条件的不同子字符串对数目。

一个 **子字符串** 是一个字符串中连续的字符。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "aba", t = "baba"
<strong>输出:</strong> 6
<strong>解释:</strong> 以下为只相差 1 个字符的 s 和 t 串的子字符串对：
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
("aba", "baba")
加粗部分分别表示 s 和 t 串选出来的子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ab", t = "bb"
<strong>输出:</strong> 3
<strong>解释:</strong> 以下为只相差 1 个字符的 s 和 t 串的子字符串对：
("ab", "bb")
("ab", "bb")
("ab", "bb")
加粗部分分别表示 s 和 t 串选出来的子字符串。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "a", t = "a"
<strong>输出:</strong> 0
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> s = "abe", t = "bbc"
<strong>输出:</strong> 10
</pre>

#### 提示:
* `1 <= s.length, t.length <= 100`
* `s` 和 `t` 都只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut ret = 0;

        for i in 0..s.len() {
            for j in 0..t.len() {
                let mut count = 0;

                for k in 0..(s.len() - i).min(t.len() - j) {
                    if s[i + k] != t[j + k] {
                        count += 1;
                    }
                    if count > 1 {
                        break;
                    } else if count == 1 {
                        ret += 1;
                    }
                }
            }
        }

        ret
    }
}
```
