# 1144. 递减元素使数组呈锯齿状
给你一个整数数组 `nums`，每次 **操作** 会从中选择一个元素并 **将该元素的值减少 1**。

如果符合下列情况之一，则数组 `A` 就是 **锯齿数组**：
* 每个偶数索引对应的元素都大于相邻的元素，即 `A[0] > A[1] < A[2] > A[3] < A[4] > ...`
* 或者，每个奇数索引对应的元素都大于相邻的元素，即 `A[0] < A[1] > A[2] < A[3] > A[4] < ...`

返回将数组 `nums` 转换为锯齿数组所需的最小操作次数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3]
<strong>输出:</strong> 2
<strong>解释:</strong> 我们可以把 2 递减到 0，或把 3 递减到 1。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [9,6,1,6,2]
<strong>输出:</strong> 4
</pre>

#### 提示:
* `1 <= nums.length <= 1000`
* `1 <= nums[i] <= 1000`

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        Self::moves_to_make_v(&nums, true).min(Self::moves_to_make_v(&nums, false))
    }

    fn moves_to_make_v(nums: &[i32], odd: bool) -> i32 {
        (odd as usize..nums.len())
            .step_by(2)
            .map(|i| {
                nums[i]
                    - nums[i]
                        .min(*nums.get(i - 1).unwrap_or(&1000) - 1)
                        .min(*nums.get(i + 1).unwrap_or(&1000) - 1)
            })
            .sum()
    }
}
```
