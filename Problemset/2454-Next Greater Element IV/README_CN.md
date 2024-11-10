# 2454. 下一个更大元素 IV
给你一个下标从 **0** 开始的非负整数数组 `nums` 。对于 `nums` 中每一个整数，你必须找到对应元素的 **第二大** 整数。

如果 `nums[j]` 满足以下条件，那么我们称它为 `nums[i]` 的 **第二大** 整数：

* `j > i`
* `nums[j] > nums[i]`
* 恰好存在 **一个** `k` 满足 `i < k < j` 且 `nums[k] > nums[i]` 。

如果不存在 `nums[j]` ，那么第二大整数为 `-1` 。

* 比方说，数组 `[1, 2, 4, 3]` 中，`1` 的第二大整数是 `4` ，`2` 的第二大整数是 `3` ，`3` 和 `4` 的第二大整数是 `-1` 。

请你返回一个整数数组 `answer` ，其中 `answer[i]`是 `nums[i]` 的第二大整数。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,0,9,6]
<strong>输出:</strong> [9,6,6,-1,-1]
<strong>解释:</strong>
下标为 0 处：2 的右边，4 是大于 2 的第一个整数，9 是第二个大于 2 的整数。
下标为 1 处：4 的右边，9 是大于 4 的第一个整数，6 是第二个大于 4 的整数。
下标为 2 处：0 的右边，9 是大于 0 的第一个整数，6 是第二个大于 0 的整数。
下标为 3 处：右边不存在大于 9 的整数，所以第二大整数为 -1 。
下标为 4 处：右边不存在大于 6 的整数，所以第二大整数为 -1 。
所以我们返回 [9,6,6,-1,-1] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [3,3]
<strong>输出:</strong> [-1,-1]
<strong>解释:</strong>
由于每个数右边都没有更大的数，所以我们返回 [-1,-1] 。
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let mut heap0 = BinaryHeap::new();
        let mut heap1 = BinaryHeap::new();
        let mut answer = vec![-1; nums.len()];

        for i in 0..nums.len() {
            while let Some(&Reverse((x, j))) = heap1.peek() {
                if x >= nums[i] {
                    break;
                }
                answer[j] = nums[i];
                heap1.pop();
            }
            while let Some(&Reverse((x, j))) = heap0.peek() {
                if x >= nums[i] {
                    break;
                }
                heap1.push(heap0.pop().unwrap());
            }
            heap0.push(Reverse((nums[i], i)));
        }

        answer
    }
}
```
