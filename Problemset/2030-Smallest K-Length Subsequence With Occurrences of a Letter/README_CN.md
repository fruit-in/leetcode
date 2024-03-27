# 2030. 含特定字母的最小子序列
给你一个字符串 `s` ，一个整数 `k` ，一个字母 `letter` 以及另一个整数 `repetition` 。

返回 `s` 中长度为 `k` 且 **字典序最小** 的子序列，该子序列同时应满足字母 `letter` 出现 **至少** `repetition` 次。生成的测试用例满足 `letter` 在 `s` 中出现 **至少** `repetition` 次。

**子序列** 是由原字符串删除一些（或不删除）字符且不改变剩余字符顺序得到的剩余字符串。

字符串 `a` 字典序比字符串 `b` 小的定义为：在 `a` 和 `b` 出现不同字符的第一个位置上，字符串 `a` 的字符在字母表中的顺序早于字符串 `b` 的字符。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leet", k = 3, letter = "e", repetition = 1
<strong>输出:</strong> "eet"
<strong>解释:</strong> 存在 4 个长度为 3 ，且满足字母 'e' 出现至少 1 次的子序列：
- "lee"（"leet"）
- "let"（"leet"）
- "let"（"leet"）
- "eet"（"leet"）
其中字典序最小的子序列是 "eet" 。
</pre>

#### 示例 2:
![](https://assets.leetcode.com/uploads/2021/09/13/smallest-k-length-subsequence.png)
<pre>
<strong>输入:</strong> s = "leetcode", k = 4, letter = "e", repetition = 2
<strong>输出:</strong> "ecde"
<strong>解释:</strong> "ecde" 是长度为 4 且满足字母 "e" 出现至少 2 次的字典序最小的子序列。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "bb", k = 2, letter = "b", repetition = 2
<strong>输出:</strong> "bb"
<strong>解释:</strong> "bb" 是唯一一个长度为 2 且满足字母 "b" 出现至少 2 次的子序列。
</pre>

#### 提示:
* <code>1 <= repetition <= k <= s.length <= 5 * 10<sup>4</sup></code>
* `s` 由小写英文字母组成
* `letter` 是一个小写英文字母，在 `s` 中至少出现 `repetition` 次

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let k = k as usize;
        let repetition = repetition as usize;
        let mut remain = s.chars().filter(|&c| c == letter).count();
        let mut count = 0;
        let mut stack = vec![];

        for (i, c) in s.chars().enumerate() {
            while *stack.last().unwrap_or(&'a') > c && s.len() - i + stack.len() > k {
                if *stack.last().unwrap() == letter {
                    if count + remain > repetition {
                        count -= 1;
                    } else {
                        break;
                    }
                }

                stack.pop();
            }

            if c == letter && count + remain == repetition && stack.len() + remain >= k {
                while stack.len() + remain > k {
                    remain += (stack.pop().unwrap() == letter) as usize;
                }

                stack.append(&mut vec![letter; remain]);

                break;
            }

            if stack.len() < k {
                count += (c == letter) as usize;
                stack.push(c);
            }

            remain -= (c == letter) as usize;
        }

        stack.into_iter().collect()
    }
}
```
