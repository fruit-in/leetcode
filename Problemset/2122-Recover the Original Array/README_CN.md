# 2122. 还原原数组
Alice 有一个下标从 **0** 开始的数组 `arr` ，由 `n` 个正整数组成。她会选择一个任意的 **正整数** `k` 并按下述方式创建两个下标从 **0** 开始的新整数数组 `lower` 和 `higher` ：

1. 对每个满足 0 <= i < n 的下标 i ，lower[i] = arr[i] - k
2. 对每个满足 0 <= i < n 的下标 i ，higher[i] = arr[i] + k

不幸地是，Alice 丢失了全部三个数组。但是，她记住了在数组 `lower` 和 `higher` 中出现的整数，但不知道每个整数属于哪个数组。请你帮助 Alice 还原原数组。

给你一个由 2n 个整数组成的整数数组 `nums` ，其中 **恰好** `n` 个整数出现在 `lower` ，剩下的出现在 `higher` ，还原并返回 **原数组** `arr` 。如果出现答案不唯一的情况，返回 **任一** 有效数组。

**注意：**生成的测试用例保证存在 **至少一个** 有效数组 `arr` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,10,6,4,8,12]
<strong>输出:</strong> [3,7,11]
<strong>解释:</strong>
如果 arr = [3,7,11] 且 k = 1 ，那么 lower = [2,6,10] 且 higher = [4,8,12] 。
组合 lower 和 higher 得到 [2,6,10,4,8,12] ，这是 nums 的一个排列。
另一个有效的数组是 arr = [5,7,9] 且 k = 3 。在这种情况下，lower = [2,4,6] 且 higher = [8,10,12] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,3,3]
<strong>输出:</strong> [2,2]
<strong>解释:</strong>
如果 arr = [2,2] 且 k = 1 ，那么 lower = [1,1] 且 higher = [3,3] 。
组合 lower 和 higher 得到 [1,1,3,3] ，这是 nums 的一个排列。
注意，数组不能是 [1,3] ，因为在这种情况下，获得 [1,1,3,3] 唯一可行的方案是 k = 0 。
这种方案是无效的，k 必须是一个正整数。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [5,435]
<strong>输出:</strong> [220]
<strong>解释:</strong>
唯一可行的组合是 arr = [220] 且 k = 215 。在这种情况下，lower = [5] 且 higher = [435] 。
</pre>

#### 提示:
* `2 * n == nums.length`
* `1 <= n <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>
* 生成的测试用例保证存在 **至少一个** 有效数组 `arr`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn recover_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut arr = vec![];

        nums.sort_unstable();

        for i in 1..nums.len() {
            if nums[i] % 2 != nums[0] % 2 || nums[i] == nums[0] {
                continue;
            }

            let k = (nums[i] - nums[0]) / 2;
            let mut count = HashMap::from([(nums[0], 1)]);

            for j in 1..nums.len() {
                if *count.get(&(nums[j] - 2 * k)).unwrap_or(&0) > 0 {
                    *count.get_mut(&(nums[j] - 2 * k)).unwrap() -= 1;
                    arr.push(nums[j] - k);
                } else {
                    *count.entry(nums[j]).or_insert(0) += 1;
                }
            }

            if arr.len() * 2 == nums.len() {
                return arr;
            } else {
                arr.clear();
            }
        }

        unreachable!()
    }
}
```
