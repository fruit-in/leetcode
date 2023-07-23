# 2016. 增量元素之间的最大差值
给你一个下标从 **0** 开始的整数数组 `nums` ，该数组的大小为 `n` ，请你计算 `nums[j] - nums[i]` 能求得的 **最大差值** ，其中 `0 <= i < j < n` 且 `nums[i] < nums[j]` 。

返回 **最大差值** 。如果不存在满足要求的 `i` 和 `j` ，返回 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [7,1,5,4]
<strong>输出:</strong> 4
<strong>解释:</strong>
最大差值出现在 i = 1 且 j = 2 时，nums[j] - nums[i] = 5 - 1 = 4 。
注意，尽管 i = 1 且 j = 0 时 ，nums[j] - nums[i] = 7 - 1 = 6 > 4 ，但 i > j 不满足题面要求，所以 6 不是有效的答案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,4,3,2]
<strong>输出:</strong> -1
<strong>解释:</strong>
不存在同时满足 i < j 和 nums[i] < nums[j] 这两个条件的 i, j 组合。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,5,2,10]
<strong>输出:</strong> 9
<strong>解释:</strong>
最大差值出现在 i = 0 且 j = 3 时，nums[j] - nums[i] = 10 - 1 = 9 。
</pre>

#### 提示:
* `n == nums.length`
* `2 <= n <= 1000`
* <code>1 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        let mut ret = -1;

        for &num in &nums[1..] {
            if num < min {
                min = num;
                max = num;
            } else if num > max {
                max = num;
                ret = ret.max(max - min);
            }
        }

        ret
    }
}
```
