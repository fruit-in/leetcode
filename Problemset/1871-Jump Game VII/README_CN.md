# 1871. 跳跃游戏 VII
给你一个下标从 **0** 开始的二进制字符串 `s` 和两个整数 `minJump` 和 `maxJump` 。一开始，你在下标 `0` 处，且该位置的值一定为 `'0'` 。当同时满足如下条件时，你可以从下标 `i` 移动到下标 `j` 处：

* `i + minJump <= j <= min(i + maxJump, s.length - 1)` 且
* `s[j] == '0'`.

如果你可以到达 `s` 的下标 `s.length - 1` 处，请你返回 `true` ，否则返回 `false` 。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "011010", minJump = 2, maxJump = 3
<strong>输出:</strong> true
<strong>解释:</strong>
第一步，从下标 0 移动到下标 3 。
第二步，从下标 3 移动到下标 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "01101110", minJump = 2, maxJump = 3
<strong>输出:</strong> false
</pre>

#### 提示:
* <code>2 <= s.length <= 10<sup>5</sup></code>
* `s[i]` 要么是 `'0'` ，要么是 `'1'`
* `s[0] == '0'`
* `1 <= minJump <= maxJump < s.length`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let mut indices = vec![0];

        for (i, _) in s.chars().enumerate().skip(1).filter(|&(_, c)| c == '0') {
            match indices.binary_search(&(i as i32 - max_jump)) {
                Err(j) if j == indices.len() || indices[j] > i as i32 - min_jump => (),
                _ => indices.push(i as i32),
            }
        }

        *indices.last().unwrap() == s.len() as i32 - 1
    }
}
```
