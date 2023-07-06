# 2381. Shifting Letters II
给你一个小写英文字母组成的字符串 `s` 和一个二维整数数组 `shifts` ，其中 <code>shifts[i] = [start<sub>i</sub>, end<sub>i</sub>, direction<sub>i</sub>]</code> 。对于每个 `i` ，将 `s` 中从下标 <code>start<sub>i</sub></code> 到下标 <code>end<sub>i</sub></code> （两者都包含）所有字符都进行移位运算，如果 <code>direction<sub>i</sub> = 1</code> 将字符向后移位，如果 <code>direction<sub>i</sub> = 0</code> 将字符向前移位。

将一个字符 **向后** 移位的意思是将这个字符用字母表中 下一个 字母替换（字母表视为环绕的，所以 `'z'` 变成 `'a'`）。类似的，将一个字符 **向前** 移位的意思是将这个字符用字母表中 **前一个** 字母替换（字母表是环绕的，所以 `'a'` 变成 `'z'` ）。

请你返回对 `s` 进行所有移位操作以后得到的最终字符串。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "abc", shifts = [[0,1,0],[1,2,1],[0,2,1]]
<strong>输出:</strong> "ace"
<strong>解释:</strong> 首先，将下标从 0 到 1 的字母向前移位，得到 s = "zac" 。
然后，将下标从 1 到 2 的字母向后移位，得到 s = "zbd" 。
最后，将下标从 0 到 2 的字符向后移位，得到 s = "ace" 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "dztz", shifts = [[0,0,0],[1,1,1]]
<strong>输出:</strong> "catz"
<strong>解释:</strong> 首先，将下标从 0 到 0 的字母向前移位，得到 s = "cztz" 。
最后，将下标从 1 到 1 的字符向后移位，得到 s = "catz" 。
</pre>

#### 提示:
* <code>1 <= s.length, shifts.length <= 5 * 10<sup>4</sup></code>
* `shifts[i].length == 3`
* <code>0 <= start<sub>i</sub> <= end<sub>i</sub> < s.length</code>
* <code>0 <= direction<sub>i</sub> <= 1</code>
* `s` 只包含小写英文字母。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut prefix_sum = vec![0; s.len()];

        for i in 0..shifts.len() {
            if shifts[i][0] > 0 {
                prefix_sum[shifts[i][0] as usize - 1] -= 2 * shifts[i][2] - 1;
            }
            prefix_sum[shifts[i][1] as usize] += 2 * shifts[i][2] - 1;
        }

        for i in (0..prefix_sum.len()).rev() {
            prefix_sum[i] += *prefix_sum.get(i + 1).unwrap_or(&0);
            s[i] = ((s[i] - b'a') as i32 + prefix_sum[i]).rem_euclid(26) as u8 + b'a';
        }

        String::from_utf8(s).unwrap()
    }
}
```
