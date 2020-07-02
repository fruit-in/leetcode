# 1422. 分割字符串的最大得分
给你一个由若干 0 和 1 组成的字符串 `s` ，请你计算并返回将该字符串分割成两个 **非空** 子字符串（即 **左** 子字符串和 **右** 子字符串）所能获得的最大得分。

「分割字符串的得分」为 **左** 子字符串中 **0** 的数量加上 **右** 子字符串中 **1** 的数量。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "011101"
<strong>输出:</strong> 5
<strong>解释:</strong>
将字符串 s 划分为两个非空子字符串的可行方案有：
左子字符串 = "0" 且 右子字符串 = "11101"，得分 = 1 + 4 = 5
左子字符串 = "01" 且 右子字符串 = "1101"，得分 = 1 + 3 = 4
左子字符串 = "011" 且 右子字符串 = "101"，得分 = 1 + 2 = 3
左子字符串 = "0111" 且 右子字符串 = "01"，得分 = 1 + 1 = 2
左子字符串 = "01110" 且 右子字符串 = "1"，得分 = 2 + 1 = 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "00111"
<strong>输出:</strong> 5
<strong>解释:</strong> 当 左子字符串 = "00" 且 右子字符串 = "111" 时，我们得到最大得分 = 2 + 3 = 5
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "1111"
<strong>输出:</strong> 3
</pre>

#### 提示:
* `2 <= s.length <= 500`
* 字符串 `s` 仅由字符 `'0'` 和 `'1'` 组成。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_score(s: String) -> i32 {
        let s = s.into_bytes();
        let mut score_l = (b'1' - s[0]) as i32;
        let mut score_r = s[1..].iter().filter(|&&ch| ch == b'1').count() as i32;
        let mut ret = score_l + score_r;

        for i in 1..(s.len() - 1) {
            match s[i] {
                b'0' => score_l += 1,
                _ => score_r -= 1,
            }
            ret = ret.max(score_l + score_r);
        }

        ret
    }
}
```
