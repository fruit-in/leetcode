# 2287. 重排字符形成目标字符串
给你两个下标从 **0** 开始的字符串 `s` 和 `target` 。你可以从 `s` 取出一些字符并将其重排，得到若干新的字符串。

从 `s` 中取出字符并重新排列，返回可以形成 `target` 的 **最大** 副本数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "ilovecodingonleetcode", target = "code"
<strong>输出:</strong> 2
<strong>解释:</strong>
对于 "code" 的第 1 个副本，选取下标为 4 、5 、6 和 7 的字符。
对于 "code" 的第 2 个副本，选取下标为 17 、18 、19 和 20 的字符。
形成的字符串分别是 "ecod" 和 "code" ，都可以重排为 "code" 。
可以形成最多 2 个 "code" 的副本，所以返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abcba", target = "abc"
<strong>输出:</strong> 1
<strong>解释:</strong>
选取下标为 0 、1 和 2 的字符，可以形成 "abc" 的 1 个副本。
可以形成最多 1 个 "abc" 的副本，所以返回 1 。
注意，尽管下标 3 和 4 分别有额外的 'a' 和 'b' ，但不能重用下标 2 处的 'c' ，所以无法形成 "abc" 的第 2 个副本。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "abbaccaddaeea", target = "aaaaa"
<strong>输出:</strong> 1
<strong>解释:</strong>
选取下标为 0 、3 、6 、9 和 12 的字符，可以形成 "aaaaa" 的 1 个副本。
可以形成最多 1 个 "aaaaa" 的副本，所以返回 1 。
</pre>

#### 提示:
* `1 <= s.length <= 100`
* `1 <= target.length <= 10`
* `s` 和 `target` 由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut count_s = [0; 26];
        let mut count_t = [0; 26];

        s.bytes().for_each(|c| count_s[(c - b'a') as usize] += 1);
        target
            .bytes()
            .for_each(|c| count_t[(c - b'a') as usize] += 1);

        (0..26)
            .filter(|&i| count_t[i] > 0)
            .map(|i| count_s[i] / count_t[i])
            .min()
            .unwrap()
    }
}
```
