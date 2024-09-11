# 792. 匹配子序列的单词数
给定字符串 `s` 和字符串数组 `words`, 返回  *`words[i]` 中是`s`的子序列的单词个数* 。

字符串的 **子序列** 是从原始字符串中生成的新字符串，可以从中删去一些字符(可以是none)，而不改变其余字符的相对顺序。

* 例如， `“ace”` 是 `“abcde”` 的子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abcde", words = ["a","bb","acd","ace"]
<strong>输出:</strong> 3
<strong>解释:</strong> 有三个是 s 的子序列的单词: "a", "acd", "ace"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
<strong>输出:</strong> 2
</pre>

#### 提示:
* <code>1 <= s.length <= 5 * 10<sup>4</sup></code>
* `1 <= words.length <= 5000`
* `1 <= words[i].length <= 50`
* `words[i]`和 `s` 都只由小写字母组成。
* `s` and `words[i]` consist of only lowercase English letters.

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut indices = vec![vec![]; 26];
        let mut ret = 0;

        for (i, c) in s.bytes().enumerate() {
            indices[(c - b'a') as usize].push(i);
        }

        for word in &words {
            let mut i = 0;
            let mut flag = true;

            for c in word.bytes() {
                match indices[(c - b'a') as usize].binary_search(&i) {
                    Err(j) if j == indices[(c - b'a') as usize].len() => {
                        flag = false;
                        break;
                    }
                    Ok(j) | Err(j) => i = indices[(c - b'a') as usize][j] + 1,
                }
            }

            ret += flag as i32;
        }

        ret
    }
}
```
