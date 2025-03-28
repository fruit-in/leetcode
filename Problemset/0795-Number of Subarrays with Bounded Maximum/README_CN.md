# 795. 区间子数组个数
给你一个整数数组 `nums` 和两个整数：`left` 及 `right` 。找出 `nums` 中连续、非空且其中最大元素在范围 `[left, right]` 内的子数组，并返回满足条件的子数组的个数。

生成的测试用例保证结果符合 **32-bit** 整数范围。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,1,4,3], left = 2, right = 3
<strong>输出:</strong> 3
<strong>解释:</strong> 满足条件的三个子数组：[2], [2, 1], [3]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [2,9,2,5,6], left = 2, right = 8
<strong>输出:</strong> 7
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>
* <code>0 <= left <= right <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
        let mut desc_stack = vec![];
        let mut countl = vec![0; nums.len()];
        let mut countr = vec![0; nums.len()];

        for i in 0..nums.len() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] < nums[i] {
                desc_stack.pop();
            }

            countl[i] = i as i32 - *desc_stack.last().unwrap_or(&-1) - 1;
            desc_stack.push(i as i32);
        }
        desc_stack.clear();
        for i in (0..nums.len()).rev() {
            while !desc_stack.is_empty() && nums[*desc_stack.last().unwrap() as usize] <= nums[i] {
                desc_stack.pop();
            }

            countr[i] = *desc_stack.last().unwrap_or(&(nums.len() as i32)) - i as i32 - 1;
            desc_stack.push(i as i32);
        }

        (0..nums.len())
            .filter(|&i| nums[i] >= left && nums[i] <= right)
            .map(|i| (countl[i] + 1) * (countr[i] + 1))
            .sum()
    }
}
```
