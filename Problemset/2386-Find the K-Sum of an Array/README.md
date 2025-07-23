# 2386. Find the K-Sum of an Array
You are given an integer array `nums` and a **positive** integer `k`. You can choose any **subsequence** of the array and sum all of its elements together.

We define the **K-Sum** of the array as the <code>k<sup>th</sup></code> **largest** subsequence sum that can be obtained (**not** necessarily distinct).

Return *the K-Sum of the array*.

A **subsequence** is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.

**Note** that the empty subsequence is considered to have a sum of `0`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,4,-2], k = 5
<strong>Output:</strong> 2
<strong>Explanation:</strong> All the possible subsequence sums that we can obtain are the following sorted in decreasing order:
- 6, 4, 4, 2, 2, 0, 0, -2.
The 5-Sum of the array is 2.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,-2,3,4,-10,12], k = 16
<strong>Output:</strong> 10
<strong>Explanation:</strong> The 16-Sum of the array is 10.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= min(2000, 2<sup>n</sup>)</code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut max_sum = nums.iter().map(|&x| x.max(0) as i64).sum::<i64>();
        let mut max_heap = BinaryHeap::from([max_sum]);
        let mut min_heap = BinaryHeap::<Reverse<i64>>::new();
        let mut sorted_abs_nums = nums.iter().map(|&x| x.abs() as i64).collect::<Vec<_>>();
        sorted_abs_nums.sort_unstable();

        for i in 0..sorted_abs_nums.len() {
            while let Some(x) = max_heap.pop() {
                if min_heap.len() == k && x <= min_heap.peek().unwrap().0 {
                    break;
                }
                min_heap.push(Reverse(x));
                if min_heap.len() < k || x - sorted_abs_nums[i] > min_heap.peek().unwrap().0 {
                    min_heap.push(Reverse(x - sorted_abs_nums[i]));
                }
                while min_heap.len() > k {
                    min_heap.pop();
                }
            }

            if min_heap.len() == k
                && (i == sorted_abs_nums.len() - 1
                    || max_sum - sorted_abs_nums[i + 1] <= min_heap.peek().unwrap().0)
            {
                return min_heap.pop().unwrap().0;
            }

            max_heap = BinaryHeap::new();

            while let Some(Reverse(x)) = min_heap.pop() {
                max_heap.push(x);
            }
        }

        unreachable!()
    }
}
```
