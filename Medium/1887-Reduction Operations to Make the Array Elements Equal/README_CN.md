# 1887. 使数组元素相等的减少操作次数
给你一个整数数组 `nums` ，你的目标是令 `nums` 中的所有元素相等。完成一次减少操作需要遵照下面的几个步骤：
1. 找出 `nums` 中的 **最大** 值。记这个值为 `largest` 并取其下标 `i` （**下标从 0 开始计数**）。如果有多个元素都是最大值，则取最小的 `i` 。
2. 找出 `nums` 中的 **下一个最大** 值，这个值 **严格小于** `largest` ，记为 `nextLargest` 。
3. 将 `nums[i]` 减少到 `nextLargest` 。

返回使 `nums` 中的所有元素相等的操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [5,1,3]
<strong>输出:</strong> 3
<strong>解释:</strong> 需要 3 次操作使 nums 中的所有元素相等：
1. largest = 5 下标为 0 。nextLargest = 3 。将 nums[0] 减少到 3 。nums = [3,1,3] 。
2. largest = 3 下标为 0 。nextLargest = 1 。将 nums[0] 减少到 1 。nums = [1,1,3] 。
3. largest = 3 下标为 2 。nextLargest = 1 。将 nums[2] 减少到 1 。nums = [1,1,1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,1,1]
<strong>输出:</strong> 0
<strong>解释:</strong> nums 中的所有元素已经是相等的。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,1,2,2,3]
<strong>输出:</strong> 4
<strong>解释:</strong> 需要 4 次操作使 nums 中的所有元素相等：
1. largest = 3 下标为 4 。nextLargest = 2 。将 nums[4] 减少到 2 。nums = [1,1,2,2,2] 。
2. largest = 2 下标为 2 。nextLargest = 1 。将 nums[2] 减少到 1 。nums = [1,1,1,2,2] 。
3. largest = 2 下标为 3 。nextLargest = 1 。将 nums[3] 减少到 1 。nums = [1,1,1,1,2] 。
4. largest = 2 下标为 4 。nextLargest = 1 。将 nums[4] 减少到 1 。nums = [1,1,1,1,1] 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 5 * 10<sup>4</sup></code>
* <code>1 <= nums[i] <= 5 * 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut count = 0;
        let mut ret = 0;
        nums.sort_unstable();
        let mut prev = nums[0];

        for &num in &nums[1..] {
            if num != prev {
                count += 1;
                prev = num;
            }

            ret += count;
        }

        ret
    }
}
```
