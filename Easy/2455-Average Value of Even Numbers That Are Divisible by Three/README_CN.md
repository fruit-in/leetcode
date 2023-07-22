# 2455. 可被三整除的偶数的平均值
给你一个由正整数组成的整数数组 `nums` ，返回其中可被 `3` 整除的所有偶数的平均值。

注意：`n` 个元素的平均值等于 `n` 个元素 **求和** 再除以 `n` ，结果 **向下取整** 到最接近的整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,6,10,12,15]
<strong>输出:</strong> 9
<strong>解释:</strong> 6 和 12 是可以被 3 整除的偶数。(6 + 12) / 2 = 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,4,7,10]
<strong>输出:</strong> 0
<strong>解释:</strong> 不存在满足题目要求的整数，所以返回 0 。
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn average_value(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().filter(|&&x| x % 6 == 0).sum::<i32>();
        let len = nums.iter().filter(|&&x| x % 6 == 0).count() as i32;

        sum.checked_div(len).unwrap_or(0)
    }
}
```
