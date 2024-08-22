# 910. 最小差值 II
给你一个整数数组 `nums`，和一个整数 `k` 。

对于每个下标 `i`（`0 <= i < nums.length`），将 `nums[i]` 变成 `nums[i] + k` 或 `nums[i] - k` 。

`nums` 的 **分数** 是 `nums` 中最大元素和最小元素的差值。

在更改每个下标对应的值之后，返回 `nums` 的最小 **分数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1], k = 0
<strong>输出:</strong> 0
<strong>解释:</strong> 分数 = max(nums) - min(nums) = 1 - 1 = 0 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,10], k = 2
<strong>输出:</strong> 6
<strong>解释:</strong> 将数组变为 [2, 8] 。分数 = max(nums) - min(nums) = 8 - 2 = 6 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,3,6], k = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 将数组变为 [4, 6, 3] 。分数 = max(nums) - min(nums) = 6 - 3 = 3 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>0 <= nums[i] <= 10<sup>4</sup></code>
* <code>0 <= k <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums.into_iter().map(|x| x + k).collect::<Vec<_>>();
        let mut ret = 0;

        nums.sort_unstable();

        if nums[n - 1] - 2 * k < nums[0] {
            ret = nums[n - 1] - nums[0];
        } else {
            for i in 1..nums.len() {
                if nums[i] - 2 * k >= nums[0] {
                    ret = nums[i - 1].max(nums[n - 1] - 2 * k) - nums[0];
                    break;
                }
            }
        }

        for i in 1..nums.len() {
            if nums[i] - 2 * k > nums[0] {
                break;
            }

            ret = ret.min(nums[i - 1].max(nums[n - 1] - 2 * k) - nums[i] + 2 * k);
        }

        ret
    }
}
```
