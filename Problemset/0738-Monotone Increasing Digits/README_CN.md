# 738. 单调递增的数字
当且仅当每个相邻位数上的数字 `x` 和 `y` 满足 `x <= y` 时，我们称这个整数是**单调递增**的。

给定一个整数 `n` ，返回 *小于或等于 `n` 的最大数字，且数字呈 **单调递增*** 。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 10
<strong>输出:</strong> 9
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 1234
<strong>输出:</strong> 1234
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 332
<strong>输出:</strong> 299
</pre>

#### 提示:
* <code>0 <= n <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut digits = n.to_string().into_bytes();
        let mut i = digits.len();

        while let Some(j) = (0..digits.len() - 1).find(|&j| digits[j] > digits[j + 1]) {
            digits[j] -= 1;
            for k in j + 1..i {
                digits[k] = b'9';
            }
            i = j + 1;
        }

        String::from_utf8(digits).unwrap().parse().unwrap()
    }
}
```
