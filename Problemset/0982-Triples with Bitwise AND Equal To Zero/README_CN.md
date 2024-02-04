# 982. 按位与为零的三元组
给你一个整数数组 `nums` ，返回其中 **按位与三元组** 的数目。

**按位与三元组** 是由下标 `(i, j, k)` 组成的三元组，并满足下述全部条件：

* `0 <= i < nums.length`
* `0 <= j < nums.length`
* `0 <= k < nums.length`
* `nums[i] & nums[j] & nums[k] == 0` ，其中 `&` 表示按位与运算符。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,3]
<strong>输出:</strong> 12
<strong>解释:</strong> 可以选出如下 i, j, k 三元组：
(i=0, j=0, k=1) : 2 & 2 & 1
(i=0, j=1, k=0) : 2 & 1 & 2
(i=0, j=1, k=1) : 2 & 1 & 1
(i=0, j=1, k=2) : 2 & 1 & 3
(i=0, j=2, k=1) : 2 & 3 & 1
(i=1, j=0, k=0) : 1 & 2 & 2
(i=1, j=0, k=1) : 1 & 2 & 1
(i=1, j=0, k=2) : 1 & 2 & 3
(i=1, j=1, k=0) : 1 & 1 & 2
(i=1, j=2, k=0) : 1 & 3 & 2
(i=2, j=0, k=1) : 3 & 2 & 1
(i=2, j=1, k=0) : 3 & 1 & 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,0,0]
<strong>输出:</strong> 27
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>0 <= nums[i] < 2<sup>16</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for i in 0..nums.len() {
            for j in 0..nums.len() {
                *count.entry(nums[i] & nums[j]).or_insert(0) += 1;
            }
        }

        for k in 0..nums.len() {
            for (x, c) in count.iter() {
                if x & nums[k] == 0 {
                    ret += c;
                }
            }
        }

        ret
    }
}
```
