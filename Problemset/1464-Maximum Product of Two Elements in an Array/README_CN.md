# 1464. 数组中两元素的最大乘积
给你一个整数数组 `nums`，请你选择数组的两个不同下标 `i` 和 `j`，使 `(nums[i]-1)*(nums[j]-1)` 取得最大值。

请你计算并返回该式的最大值。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,4,5,2]
<strong>输出:</strong> 12
<strong>解释:</strong> 如果选择下标 i=1 和 j=2（下标从 0 开始），则可以获得最大值，(nums[1]-1)*(nums[2]-1) = (4-1)*(5-1) = 3*4 = 12 。 
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,5,4,5]
<strong>输出:</strong> 16
<strong>解释:</strong> 选择下标 i=1 和 j=3（下标从 0 开始），则可以获得最大值 (5-1)*(5-1) = 16 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [3,7]
<strong>输出:</strong> 12
</pre>

#### 提示:
* `2 <= nums.length <= 500`
* `1 <= nums[i] <= 10^3`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut i = 1;
        let mut j = 0;

        for k in 2..nums.len() {
            if nums[k] > nums[i] {
                i = k;
            }
        }
        for k in 1..nums.len() {
            if nums[k] > nums[j] && k != i {
                j = k;
            }
        }

        (nums[i] - 1) * (nums[j] - 1)
    }
}
```
