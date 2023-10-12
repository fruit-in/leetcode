# 300. 最长递增子序列
给你一个整数数组 `nums` ，找到其中最长严格递增子序列的长度。

**子序列** 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，`[3,6,2,7]` 是数组 `[0,3,1,6,2,2,7]` 的子序列。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,9,2,5,3,7,101,18]
<strong>输出:</strong> 4
<strong>解释:</strong> 最长递增子序列是 [2,3,7,101]，因此长度为 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [0,1,0,3,2,3]
<strong>输出:</strong> 4
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [7,7,7,7,7,7,7]
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= nums.length <= 2500`
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

#### 进阶:
* 你能将算法的时间复杂度降低到 `O(n log(n))` 吗?

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = vec![-10001];

        for &num in &nums {
            if num > *stack.last().unwrap() {
                stack.push(num);
            } else if let Err(i) = stack.binary_search(&num) {
                stack[i] = num;
            }
        }

        stack.len() as i32 - 1
    }
}
```
