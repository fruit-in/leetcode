# 1012. 至少有 1 位重复的数字
给定正整数 `n`，返回在 `[1, n]` 范围内具有 **至少 1 位** 重复数字的正整数的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 20
<strong>输出:</strong> 1
<strong>解释:</strong> 具有至少 1 位重复数字的正数（<= 20）只有 11 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 100
<strong>输出:</strong> 10
<strong>解释:</strong> 具有至少 1 位重复数字的正数（<= 100）有 11，22，33，44，55，66，77，88，99 和 100 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 1000
<strong>输出:</strong> 262
</pre>

#### 提示:
* <code>1 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_dup_digits_at_most_n(n: i32) -> i32 {
        let digits = n
            .to_string()
            .bytes()
            .map(|ch| (ch - b'0') as i32)
            .collect::<Vec<_>>();
        let mut ret = n;

        for i in 0..digits.len() - 1 {
            let mut x = 9;

            for j in (10 - i as i32)..=9 {
                x *= j;
            }

            ret -= x;
        }

        for i in 0..digits.len() {
            let mut count = (i == 0) as i32;

            for j in 0..i {
                if digits[j] < digits[i] {
                    count += 1;
                }
            }

            let mut x = digits[i] - count;

            for j in (11 - digits.len() as i32)..(10 - i as i32) {
                x *= j;
            }

            ret -= x;

            if digits[..i].contains(&digits[i]) {
                break;
            }

            if i == digits.len() - 1 {
                ret -= 1;
            }
        }

        ret
    }
}
```
