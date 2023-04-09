# 1814. 统计一个数组中好对子的数目
给你一个数组 `nums` ，数组中只包含非负整数。定义 `rev(x)` 的值为将整数 `x` 各个数字位反转得到的结果。比方说 `rev(123) = 321` ， `rev(120) = 21` 。我们称满足下面条件的下标对 `(i, j)` 是 **好的** ：

* `0 <= i < j < nums.length`
* `nums[i] + rev(nums[j]) == nums[j] + rev(nums[i])`

请你返回好下标对的数目。由于结果可能会很大，请将结果对 <code>10<sup>9</sup> + 7</code> **取余** 后返回。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [42,11,1,97]
<strong>输出:</strong> 2
<strong>解释:</strong> 两个坐标对为：
 - (0,3)：42 + rev(97) = 42 + 79 = 121, 97 + rev(42) = 97 + 24 = 121 。
 - (1,2)：11 + rev(1) = 11 + 1 = 12, 1 + rev(11) = 1 + 11 = 12 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [13,10,35,24,76]
<strong>输出:</strong> 4
</pre>

#### 提示:
* <code>1 <= nums.length <=10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();

        for &num in &nums {
            count
                .entry(num - Self::rev(num))
                .and_modify(|x| *x += 1)
                .or_insert(1_i64);
        }

        (count.into_values().map(|x| (x - 1) * x / 2).sum::<i64>() % 1_000_000_007) as i32
    }

    pub fn rev(x: i32) -> i32 {
        let mut x = x;
        let mut ret = 0;

        while x > 0 {
            ret = ret * 10 + x % 10;
            x /= 10;
        }

        ret
    }
}
```
