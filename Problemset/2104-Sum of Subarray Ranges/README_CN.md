# 2104. 子数组范围和
给你一个整数数组 `nums` 。`nums` 中，子数组的 **范围** 是子数组中最大元素和最小元素的差值。

返回 `nums` 中 **所有** 子数组范围的 **和** 。

子数组是数组中一个连续 **非空** 的元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> 4
<strong>解释:</strong> nums 的 6 个子数组如下所示：
[1]，范围 = 最大 - 最小 = 1 - 1 = 0
[2]，范围 = 2 - 2 = 0
[3]，范围 = 3 - 3 = 0
[1,2]，范围 = 2 - 1 = 1
[2,3]，范围 = 3 - 2 = 1
[1,2,3]，范围 = 3 - 1 = 2
所有范围的和是 0 + 0 + 0 + 1 + 1 + 2 = 4
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,3,3]
<strong>输出:</strong> 4
<strong>解释:</strong> nums 的 6 个子数组如下所示：
[1]，范围 = 最大 - 最小 = 1 - 1 = 0
[3]，范围 = 3 - 3 = 0
[3]，范围 = 3 - 3 = 0
[1,3]，范围 = 3 - 1 = 2
[3,3]，范围 = 3 - 3 = 0
[1,3,3]，范围 = 3 - 1 = 2
所有范围的和是 0 + 0 + 0 + 2 + 0 + 2 = 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [4,-2,-3,4,1]
<strong>输出:</strong> 59
<strong>解释:</strong> nums 中所有子数组范围的和是 59
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

**进阶：**你可以设计一种时间复杂度为 `O(n)` 的解决方案吗？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut count_as_max = vec![0; n];
        let mut count_as_min = vec![0; n];

        let mut dec_stack = vec![];
        let mut inc_stack = vec![];
        for i in 0..n {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 < nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 > nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] = i as i64 - dec_stack.last().unwrap_or(&(-1, 0)).0;
            count_as_min[i] = i as i64 - inc_stack.last().unwrap_or(&(-1, 0)).0;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        dec_stack = vec![];
        inc_stack = vec![];
        for i in (0..n).rev() {
            while dec_stack.last().unwrap_or(&(0, i32::MAX)).1 <= nums[i] {
                dec_stack.pop();
            }
            while inc_stack.last().unwrap_or(&(0, i32::MIN)).1 >= nums[i] {
                inc_stack.pop();
            }

            count_as_max[i] *= dec_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            count_as_min[i] *= inc_stack.last().unwrap_or(&(n as i64, 0)).0 - i as i64;
            dec_stack.push((i as i64, nums[i]));
            inc_stack.push((i as i64, nums[i]));
        }

        (0..n)
            .map(|i| (count_as_max[i] - count_as_min[i]) * nums[i] as i64)
            .sum()
    }
}
```
