# 2163. 删除元素后和的最小差值
给你一个下标从 **0** 开始的整数数组 `nums` ，它包含 `3 * n` 个元素。

你可以从 `nums` 中删除 **恰好** `n` 个元素，剩下的 `2 * n` 个元素将会被分成两个 **相同大小** 的部分。

* 前面 `n` 个元素属于第一部分，它们的和记为 <code>sum<sub>first</sub></code> 。
* 后面 `n` 个元素属于第二部分，它们的和记为 <code>sum<sub>second</sub></code> 。

两部分和的 **差值** 记为 <code>sum<sub>first</sub> - sum<sub>second</sub></code> 。

* 比方说，<code>sum<sub>first</sub> = 3</code> 且 <code>sum<sub>second</sub> = 2</code> ，它们的差值为 `1` 。
* 再比方，<code>sum<sub>first</sub> = 2</code> 且 <code>sum<sub>second</sub> = 3</code> ，它们的差值为 `-1` 。

请你返回删除 `n` 个元素之后，剩下两部分和的 **差值的最小值** 是多少。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [3,1,2]
<strong>输出:</strong> -1
<strong>解释:</strong> nums 有 3 个元素，所以 n = 1 。
所以我们需要从 nums 中删除 1 个元素，并将剩下的元素分成两部分。
- 如果我们删除 nums[0] = 3 ，数组变为 [1,2] 。两部分和的差值为 1 - 2 = -1 。
- 如果我们删除 nums[1] = 1 ，数组变为 [3,2] 。两部分和的差值为 3 - 2 = 1 。
- 如果我们删除 nums[2] = 2 ，数组变为 [3,1] 。两部分和的差值为 3 - 1 = 2 。
两部分和的最小差值为 min(-1,1,2) = -1 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [7,9,5,8,1,3]
<strong>输出:</strong> 1
<strong>解释:</strong> n = 2 。所以我们需要删除 2 个元素，并将剩下元素分为 2 部分。
如果我们删除元素 nums[2] = 5 和 nums[3] = 8 ，剩下元素为 [7,9,1,3] 。和的差值为 (7+9) - (1+3) = 12 。
为了得到最小差值，我们应该删除 nums[1] = 9 和 nums[4] = 1 ，剩下的元素为 [7,5,8,3] 。和的差值为 (7+5) - (8+3) = 1 。
观察可知，最优答案为 1 。
</pre>

#### 提示:
* `nums.length == 3 * n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let n = nums.len() / 3;
        let mut heap = BinaryHeap::new();
        let mut min_sum1 = vec![0; n + 1];
        let mut max_sum2 = vec![0; n + 1];

        for i in 0..n {
            heap.push(nums[i]);
            min_sum1[0] += nums[i];
        }
        for i in 1..=n {
            min_sum1[i] = min_sum1[i - 1];
            if nums[n + i - 1] < *heap.peek().unwrap() {
                min_sum1[i] -= heap.pop().unwrap();
                heap.push(nums[n + i - 1]);
                min_sum1[i] += nums[n + i - 1];
            }
        }

        heap.clear();

        for i in n * 2..nums.len() {
            heap.push(-nums[i]);
            max_sum2[n] += nums[i];
        }
        for i in (0..n).rev() {
            max_sum2[i] = max_sum2[i + 1];
            if nums[n + i] > -*heap.peek().unwrap() {
                max_sum2[i] -= -heap.pop().unwrap();
                heap.push(-nums[n + i]);
                max_sum2[i] += nums[n + i];
            }
        }

        (0..=n).map(|i| min_sum1[i] - max_sum2[i]).min().unwrap()
    }
}
```
