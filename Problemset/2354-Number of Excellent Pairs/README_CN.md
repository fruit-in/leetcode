# 2354. 优质数对的数目
给你一个下标从 **0** 开始的正整数数组 `nums` 和一个正整数 `k` 。

如果满足下述条件，则数对 `(num1, num2)` 是 **优质数对** ：

* `num1` 和 `num2` **都** 在数组 `nums` 中存在。
* `num1 OR num2` 和 `num1 AND num2` 的二进制表示中值为 **1** 的位数之和大于等于 `k` ，其中 `OR` 是按位 **或** 操作，而 `AND` 是按位 **与** 操作。

返回 **不同** 优质数对的数目。

如果 `a != c` 或者 `b != d` ，则认为 `(a, b)` 和 `(c, d)` 是不同的两个数对。例如，`(1, 2)` 和 `(2, 1)` 不同。

**注意：**如果 `num1` 在数组中至少出现 一次 ，则满足 `num1 == num2` 的数对 `(num1, num2)` 也可以是优质数对。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,1], k = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 有如下几个优质数对：
- (3, 3)：(3 AND 3) 和 (3 OR 3) 的二进制表示都等于 (11) 。值为 1 的位数和等于 2 + 2 = 4 ，大于等于 k = 3 。
- (2, 3) 和 (3, 2)： (2 AND 3) 的二进制表示等于 (10) ，(2 OR 3) 的二进制表示等于 (11) 。值为 1 的位数和等于 1 + 2 = 3 。
- (1, 3) 和 (3, 1)： (1 AND 3) 的二进制表示等于 (01) ，(1 OR 3) 的二进制表示等于 (11) 。值为 1 的位数和等于 1 + 2 = 3 。
所以优质数对的数目是 5 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [5,1,1], k = 10
<strong>输出:</strong> 0
<strong>解释:</strong> 该数组中不存在优质数对。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* `1 <= k <= 60`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn count_excellent_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut count = vec![0; 32.min(k + 1)];
        let mut ret = 0;

        for num in nums.iter().collect::<HashSet<_>>().into_iter() {
            count[k.min(num.count_ones() as usize)] += 1;
        }

        for i in 1..count.len() {
            for j in k - i..count.len() {
                ret += count[i] * count[j];
            }
        }

        ret
    }
}
```
