# 2486. 追加字符以获得子序列
给你两个仅由小写英文字母组成的字符串 `s` 和 `t` 。

现在需要通过向 `s` 末尾追加字符的方式使 `t` 变成 `s` 的一个 **子序列** ，返回需要追加的最少字符数。

子序列是一个可以由其他字符串删除部分（或不删除）字符但不改变剩下字符顺序得到的字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "coaching", t = "coding"
<strong>输出:</strong> 4
<strong>解释:</strong> 向 s 末尾追加字符串 "ding" ，s = "coachingding" 。
现在，t 是 s ("coachingding") 的一个子序列。
可以证明向 s 末尾追加任何 3 个字符都无法使 t 成为 s 的一个子序列。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcde", t = "a"
<strong>输出:</strong> 0
<strong>解释:</strong> t 已经是 s ("abcde") 的一个子序列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "z", t = "abcde"
<strong>输出:</strong> 5
<strong>解释:</strong> 向 s 末尾追加字符串 "abcde" ，s = "zabcde" 。
现在，t 是 s ("zabcde") 的一个子序列。
可以证明向 s 末尾追加任何 4 个字符都无法使 t 成为 s 的一个子序列。
</pre>

#### 提示:
* <code>1 <= s.length, t.length <= 10<sup>5</sup></code>
* `s` 和 `t` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut i = 0;

        for j in 0..s.len() {
            if i >= t.len() {
                break;
            } else if t[i] == s[j] {
                i += 1;
            }
        }

        (t.len() - i) as i32
    }
}
```
