# 2615. 等值距离和
给你一个下标从 **0** 开始的整数数组 `nums` 。现有一个长度等于 `nums.length` 的数组 `arr` 。对于满足 `nums[j] == nums[i]` 且 `j != i` 的所有 `j` ，`arr[i]` 等于所有 `|i - j|` 之和。如果不存在这样的 `j` ，则令 `arr[i]` 等于 `0` 。

返回数组 `arr` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,1,1,2]
<strong>输出:</strong> [5,0,3,4,0]
<strong>解释:</strong>
i = 0 ，nums[0] == nums[2] 且 nums[0] == nums[3] 。因此，arr[0] = |0 - 2| + |0 - 3| = 5 。
i = 1 ，arr[1] = 0 因为不存在值等于 3 的其他下标。
i = 2 ，nums[2] == nums[0] 且 nums[2] == nums[3] 。因此，arr[2] = |2 - 0| + |2 - 3| = 3 。
i = 3 ，nums[3] == nums[0] 且 nums[3] == nums[2] 。因此，arr[3] = |3 - 0| + |3 - 2| = 4 。
i = 4 ，arr[4] = 0 因为不存在值等于 2 的其他下标。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,5,3]
<strong>输出:</strong> [0,0,0]
<strong>解释:</strong> 因为 nums 中的元素互不相同，对于所有 i ，都有 arr[i] = 0 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        let mut count = HashMap::new();
        let mut indices_sum = HashMap::new();
        let mut arr = vec![0; nums.len()];

        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] = count[&nums[i]] * i as i64 - indices_sum[&nums[i]];
        }

        count.clear();
        indices_sum.clear();

        for i in (0..nums.len()).rev() {
            *count.entry(nums[i]).or_insert(0) += 1;
            *indices_sum.entry(nums[i]).or_insert(0) += i as i64;
            arr[i] += indices_sum[&nums[i]] - count[&nums[i]] * i as i64;
        }

        arr
    }
}
```
