# 503. 下一个更大元素 II
给定一个循环数组 `nums` （ `nums[nums.length - 1]` 的下一个元素是 `nums[0]` ），返回 `nums` *中每个元素的 **下一个更大元素*** 。

数字 `x` 的 **下一个更大的元素** 是按数组遍历顺序，这个数字之后的第一个比它更大的数，这意味着你应该循环地搜索它的下一个更大的数。如果不存在，则输出 `-1` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,1]
<strong>输出:</strong> [2,-1,2]
<strong>解释:</strong> 第一个 1 的下一个更大的数是 2；
数字 2 找不到下一个更大的数；
第二个 1 的下一个最大的数需要循环搜索，结果也是 2。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,4,3]
<strong>输出:</strong> [2,3,4,-1,4]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>4</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = vec![];
        let mut ret = vec![-1; nums.len()];

        for i in (0..nums.len()).chain(0..nums.len()) {
            while !stack.is_empty() && nums[i] > nums[*stack.last().unwrap()] {
                ret[stack.pop().unwrap()] = nums[i];
            }

            if ret[i] == -1 {
                stack.push(i);
            }
        }

        ret
    }
}
```
