# 2535. 数组元素和与数字和的绝对差
给你一个正整数数组 `nums` 。

* **元素和** 是 `nums` 中的所有元素相加求和。
* **数字和** 是 `nums` 中每一个元素的每一数位（重复数位需多次求和）相加求和。

返回 **元素和** 与 **数字和** 的绝对差。

**注意：**两个整数 `x` 和 `y` 的绝对差定义为 `|x - y|` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,15,6,3]
<strong>输出:</strong> 9
<strong>解释:</strong>
nums 的元素和是 1 + 15 + 6 + 3 = 25 。
nums 的数字和是 1 + 1 + 5 + 6 + 3 = 16 。
元素和与数字和的绝对差是 |25 - 16| = 9 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4]
<strong>输出:</strong> 0
<strong>解释:</strong>
nums 的元素和是 1 + 2 + 3 + 4 = 10 。
nums 的数字和是 1 + 2 + 3 + 4 = 10 。
元素和与数字和的绝对差是 |10 - 10| = 0 。
</pre>

#### 提示:
* `1 <= nums.length <= 2000`
* `1 <= nums[i] <= 2000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut element_sum = 0;
        let mut digit_sum = 0;

        for i in 0..nums.len() {
            element_sum += nums[i];
            while nums[i] > 0 {
                digit_sum += nums[i] % 10;
                nums[i] /= 10;
            }
        }

        (element_sum - digit_sum).abs()
    }
}
```
