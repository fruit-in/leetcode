# 1425. 带限制的子序列和
给你一个整数数组 `nums` 和一个整数 `k` ，请你返回 **非空** 子序列元素和的最大值，子序列需要满足：子序列中每两个 **相邻** 的整数 `nums[i]` 和 `nums[j]` ，它们在原数组中的下标 `i` 和 `j` 满足 `i < j` 且 `j - i <= k` 。

数组的子序列定义为：将数组中的若干个数字删除（可以删除 0 个数字），剩下的数字按照原本的顺序排布。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [10,2,-10,5,20], k = 2
<strong>输出:</strong> 37
<strong>解释:</strong> 子序列为 [10, 2, 5, 20] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [-1,-2,-3], k = 1
<strong>输出:</strong> -1
<strong>解释:</strong> 子序列必须是非空的，所以我们选择最大的数字。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [10,-2,-10,-5,20], k = 2
<strong>输出:</strong> 23
<strong>解释:</strong> 子序列为 [10, -2, -5, 20] 。
</pre>

#### 提示:
* <code>1 <= k <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut ret = *nums.iter().max().unwrap();

        for i in 0..nums.len() {
            if i - deque.front().unwrap_or(&(i, 0)).0 > k {
                deque.pop_front();
            }

            let x = nums[i] + deque.front().unwrap_or(&(0, 0)).1;

            if x > 0 {
                while deque.back().unwrap_or(&(0, i32::MAX)).1 <= x {
                    deque.pop_back();
                }
                deque.push_back((i, x));
                ret = ret.max(x);
            }
        }

        ret
    }
}
```
