# 670. 最大交换
给定一个非负整数，你**至多**可以交换一次数字中的任意两位。返回你能得到的最大值。

#### 示例 1:
<pre>
<strong>输入:</strong> 2736
<strong>输出:</strong> 7236
<strong>解释:</strong> 交换数字2和数字7。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 9973
<strong>输出:</strong> 9973
<strong>解释:</strong> 不需要交换。
</pre>

#### 注意:
1. 给定数字的范围是 [0, 10<sup>8</sup>]

## 题解 (Rust)

### 1. 贪心
```Rust
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num.to_string().into_bytes();
        let mut last = [0; 10];

        for i in 0..digits.len() {
            last[(digits[i] - b'0') as usize] = i;
        }

        for i in 0..digits.len() {
            for j in (((digits[i] - b'0') as usize + 1)..=9).rev() {
                if last[j] > i {
                    digits.swap(i, last[j]);
                    return String::from_utf8(digits).unwrap().parse().unwrap();
                }
            }
        }

        num
    }
}
```
