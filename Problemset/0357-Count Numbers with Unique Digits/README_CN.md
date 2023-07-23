# 357. 计算各个位数不同的数字个数
给定一个**非负**整数 n，计算各位数字都不同的数字 x 的个数，其中 0 ≤ x < 10<sup>n</sup> 。

#### 示例:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 91
<strong>解释:</strong> 答案应为除去 11,22,33,44,55,66,77,88,99 外，在 [0,100) 区间内的所有数字。
</pre>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let factorials = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];
        let mut n = n.min(10) as usize;
        let mut ret = 1;

        while n > 0 {
            ret += 9 * factorials[9] / factorials[10 - n];
            n -= 1;
        }

        ret
    }
}
```
