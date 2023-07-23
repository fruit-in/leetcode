# 485. 最大连续1的个数
给定一个二进制数组， 计算其中最大连续1的个数。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,1,0,1,1,1]
<strong>输出:</strong> 3
<strong>解释:</strong> 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3.
</pre>

#### 注意:
* 输入的数组只包含 ```0``` 和```1```。
* 输入数组的长度是正整数，且不超过 10,000。

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut m = 0;
        for n in nums {
            if n == 1 {
                i += 1;
            } else {
                m = m.max(i);
                i = 0;
            }
        }
        m.max(i)
    }
}
```
