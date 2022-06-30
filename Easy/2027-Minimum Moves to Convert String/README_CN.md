# 2027. 转换字符串的最少操作次数
给你一个字符串 `s` ，由 `n` 个字符组成，每个字符不是 `'X'` 就是 `'O'` 。

一次 **操作** 定义为从 `s` 中选出 **三个连续字符** 并将选中的每个字符都转换为 `'O'` 。注意，如果字符已经是 `'O'` ，只需要保持 **不变** 。

返回将 `s` 中所有字符均转换为 `'O'` 需要执行的 **最少** 操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "XXX"
<strong>输出:</strong> 1
<strong>解释:</strong> XXX -> OOO
一次操作，选中全部 3 个字符，并将它们转换为 'O' 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "XXOX"
<strong>输出:</strong> 2
<strong>解释:</strong> XXOX -> OOOX -> OOOO
第一次操作，选择前 3 个字符，并将这些字符转换为 'O' 。
然后，选中后 3 个字符，并执行转换。最终得到的字符串全由字符 'O' 组成。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "OOOO"
<strong>输出:</strong> 0
<strong>解释:</strong> s 中不存在需要转换的 'X' 。
</pre>

#### 提示:
* `3 <= s.length <= 1000`
* `s[i]` 为 `'X'` 或 `'O'`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let s = s.as_bytes();
        let mut i = 0;
        let mut ret = 0;

        while i < s.len() {
            if s[i] == b'X' {
                i += 2;
                ret += 1;
            }
            i += 1;
        }

        ret
    }
}
```
