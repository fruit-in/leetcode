# 1679. K 和数对的最大数目
给你一个整数数组 `nums` 和一个整数 `k` 。

每一步操作中，你需要从数组中选出和为 `k` 的两个整数，并将它们移出数组。

返回你可以对数组执行的最大操作数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,4], k = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 开始时 nums = [1,2,3,4]：
- 移出 1 和 4 ，之后 nums = [2,3]
- 移出 2 和 3 ，之后 nums = []
不再有和为 5 的数对，因此最多执行 2 次操作。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,1,3,4,3], k = 6
<strong>输出:</strong> 1
<strong>解释:</strong> 开始时 nums = [3,1,3,4,3]：
- 移出前两个 3 ，之后nums = [1,4,3]
不再有和为 6 的数对，因此最多执行 1 次操作。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut ret = 0;

        for num in nums {
            if *count.get(&(k - num)).unwrap_or(&0) > 0 {
                *count.get_mut(&(k - num)).unwrap() -= 1;
                ret += 1;
            } else {
                *count.entry(num).or_insert(0) += 1;
            }
        }

        ret
    }
}
```
