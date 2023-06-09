# 2606. 找到最大开销的子字符串
给你一个字符串 `s` ，一个字符 **互不相同** 的字符串 `chars` 和一个长度与 `chars` 相同的整数数组 `vals` 。

**子字符串的开销** 是一个子字符串中所有字符对应价值之和。空字符串的开销是 `0` 。

**字符的价值** 定义如下：

* 如果字符不在字符串 `chars` 中，那么它的价值是它在字母表中的位置（下标从 **1** 开始）。
    * 比方说，`'a'` 的价值为 `1` ，`'b'` 的价值为 `2` ，以此类推，`'z'` 的价值为 `26` 。
* 否则，如果这个字符在 `chars` 中的位置为 `i` ，那么它的价值就是 `vals[i]` 。

请你返回字符串 `s` 的所有子字符串中的最大开销。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "adaa", chars = "d", vals = [-1000]
<strong>输出:</strong> 2
<strong>解释:</strong> 字符 "a" 和 "d" 的价值分别为 1 和 -1000 。
最大开销子字符串是 "aa" ，它的开销为 1 + 1 = 2 。
2 是最大开销。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "abc", chars = "abc", vals = [-1,-1,-1]
<strong>输出:</strong> 0
<strong>解释:</strong> 字符 "a" ，"b" 和 "c" 的价值分别为 -1 ，-1 和 -1 。
最大开销子字符串是 "" ，它的开销为 0 。
0 是最大开销。
</pre>

#### 提示:
* <code>1 <= s.length <= 10<sup>5</sup></code>
* `s` 只包含小写英文字母。
* `1 <= chars.length <= 26`
* `chars` 只包含小写英文字母，且 **互不相同** 。
* `vals.length == chars.length`
* `-1000 <= vals[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut char_vals = (1..=26).collect::<Vec<_>>();
        let mut prefix_sum = 0;
        let mut min_sum = 0;
        let mut ret = 0;

        for (i, c) in chars.bytes().enumerate() {
            char_vals[(c - b'a') as usize] = vals[i];
        }

        for c in s.bytes() {
            prefix_sum += char_vals[(c - b'a') as usize];
            min_sum = min_sum.min(prefix_sum);
            ret = ret.max(prefix_sum - min_sum);
        }

        ret
    }
}
```
