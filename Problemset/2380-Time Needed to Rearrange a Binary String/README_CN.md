# 2380. 二进制字符串重新安排顺序需要的时间
给你一个二进制字符串 `s` 。在一秒之中，**所有** 子字符串 `"01"` **同时** 被替换成 `"10"` 。这个过程持续进行到没有 `"01"` 存在。

请你返回完成这个过程所需要的秒数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "0110101"
<strong>输出:</strong> 4
<strong>解释:</strong>
一秒后，s 变成 "1011010" 。
再过 1 秒后，s 变成 "1101100" 。
第三秒过后，s 变成 "1110100" 。
第四秒后，s 变成 "1111000" 。
此时没有 "01" 存在，整个过程花费 4 秒。
所以我们返回 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "11100"
<strong>输出:</strong> 0
<strong>解释:</strong>
s 中没有 "01" 存在，整个过程花费 0 秒。
所以我们返回 0 。
</pre>

#### 提示:
* `1 <= s.length <= 1000`
* `s[i]` 要么是 `'0'` ，要么是 `'1'` 。

#### 进阶:
你能以 O(n) 的时间复杂度解决这个问题吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn seconds_to_remove_occurrences(s: String) -> i32 {
        let mut count0 = 0;
        let mut wait = 0;

        for c in s.trim_start_matches('1').trim_end_matches('0').chars() {
            if c == '0' {
                count0 += 1;
                wait -= 1;
            } else {
                wait = (wait + 1).max(0);
            }
        }

        count0 + wait
    }
}
```
