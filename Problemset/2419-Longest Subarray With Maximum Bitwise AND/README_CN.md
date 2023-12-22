# 2419. 按位与最大的最长子数组
给你一个长度为 `n` 的整数数组 `nums` 。

考虑 `nums` 中进行 **按位与（bitwise AND）**运算得到的值 **最大** 的 **非空** 子数组。

* 换句话说，令 `k` 是 `nums` **任意** 子数组执行按位与运算所能得到的最大值。那么，只需要考虑那些执行一次按位与运算后等于 `k` 的子数组。

返回满足要求的 **最长** 子数组的长度。

数组的按位与就是对数组中的所有数字进行按位与运算。

**子数组** 是数组中的一个连续元素序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,3,2,2]
<strong>输出:</strong> 2
<strong>解释:</strong>
子数组按位与运算的最大值是 3 。
能得到此结果的最长子数组是 [3,3]，所以返回 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 1
<strong>解释:</strong>
子数组按位与运算的最大值是 4 。
能得到此结果的最长子数组是 [4]，所以返回 1 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_num = nums[0];
        let mut count = 1;
        let mut ret = 1;

        for i in 1..nums.len() {
            if nums[i] > max_num {
                max_num = nums[i];
                count = 1;
                ret = 1;
            } else if nums[i] == max_num && nums[i] != nums[i - 1] {
                count = 1;
            } else if nums[i] == max_num {
                count += 1;
                ret = ret.max(count);
            }
        }

        ret
    }
}
```
