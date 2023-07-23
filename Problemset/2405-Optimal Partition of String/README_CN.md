# 2405. 子字符串的最优划分
给你一个字符串 `s` ，请你将该字符串划分成一个或多个 **子字符串** ，并满足每个子字符串中的字符都是 **唯一** 的。也就是说，在单个子字符串中，字母的出现次数都不超过 **一次** 。

满足题目要求的情况下，返回 **最少** 需要划分多少个子字符串。

注意，划分后，原字符串中的每个字符都应该恰好属于一个子字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abacaba"
<strong>输出:</strong> 4
<strong>解释:</strong>
两种可行的划分方法分别是 ("a","ba","cab","a") 和 ("ab","a","ca","ba") 。
可以证明最少需要划分 4 个子字符串。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "ssssss"
<strong>输出:</strong> 6
<strong>解释:</strong>
只存在一种可行的划分方法 ("s","s","s","s","s","s") 。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 仅由小写英文字母组成

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut mask = 0;
        let mut ret = 1;

        for c in s.bytes() {
            if (1 << (c - b'a')) & mask != 0 {
                mask = 0;
                ret += 1;
            }

            mask |= 1 << (c - b'a');
        }

        ret
    }
}
```
