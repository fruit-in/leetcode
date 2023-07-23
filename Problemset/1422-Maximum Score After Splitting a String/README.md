# 1422. Maximum Score After Splitting a String
Given a string `s` of zeros and ones, *return the maximum score after splitting the string into two **non-empty** substrings* (i.e. **left** substring and **right** substring).

The score after splitting a string is the number of **zeros** in the **left** substring plus the number of **ones** in the **right** substring.

#### Example 1:
<pre>
<strong>Input:</strong> s = "011101"
<strong>Output:</strong> 5
<strong>Explanation:</strong>
All possible ways of splitting s into two non-empty substrings are:
left = "0" and right = "11101", score = 1 + 4 = 5
left = "01" and right = "1101", score = 1 + 3 = 4
left = "011" and right = "101", score = 1 + 2 = 3
left = "0111" and right = "01", score = 1 + 1 = 2
left = "01110" and right = "1", score = 2 + 1 = 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> s = "00111"
<strong>Output:</strong> 5
<strong>Explanation:</strong> When left = "00" and right = "111", we get the maximum score = 2 + 3 = 5
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> s = "1111"
<strong>Output:</strong> 3
</pre>

#### Constraints:
* `2 <= s.length <= 500`
* The string `s` consists of characters '0' and '1' only.

## Solutions (Rust)

### 1. Solution
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
