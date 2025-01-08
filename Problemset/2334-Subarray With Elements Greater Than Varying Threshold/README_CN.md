# 2334. 元素值大于变化阈值的子数组
给你一个整数数组 `nums` 和一个整数 `threshold` 。

找到长度为 `k` 的 `nums` 子数组，满足数组中 **每个** 元素都 **大于** `threshold / k` 。

请你返回满足要求的 **任意** 子数组的 **大小** 。如果没有这样的子数组，返回 `-1` 。

**子数组** 是数组中一段连续非空的元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,4,3,1], threshold = 6
<strong>输出:</strong> 3
<strong>解释:</strong> 子数组 [3,4,3] 大小为 3 ，每个元素都大于 6 / 3 = 2 。
注意这是唯一合法的子数组。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [6,5,6,5,8], threshold = 7
<strong>输出:</strong> 1
<strong>解释:</strong> 子数组 [8] 大小为 1 ，且 8 > 7 / 1 = 7 。所以返回 1 。
注意子数组 [6,5] 大小为 2 ，每个元素都大于 7 / 2 = 3.5 。
类似的，子数组 [6,5,6] ，[6,5,6,5] ，[6,5,6,5,8] 都是符合条件的子数组。
所以返回 2, 3, 4 和 5 都可以。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i], threshold <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut sublens = vec![0; nums.len()];
        let mut asc_stack = vec![];

        for i in 0..nums.len() {
            while asc_stack.last().unwrap_or(&(-1, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] = i as i32 - asc_stack.last().unwrap_or(&(-1, 0)).0;
            asc_stack.push((i as i32, nums[i]));
        }

        asc_stack.clear();

        for i in (0..nums.len()).rev() {
            while asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).1 >= nums[i] {
                asc_stack.pop();
            }
            sublens[i] += asc_stack.last().unwrap_or(&(nums.len() as i32, 0)).0 - i as i32 - 1;
            asc_stack.push((i as i32, nums[i]));

            if nums[i] > threshold / sublens[i] {
                return sublens[i];
            }
        }

        -1
    }
}
```
