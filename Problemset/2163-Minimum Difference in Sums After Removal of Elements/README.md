# 2163. Minimum Difference in Sums After Removal of Elements
You are given a **0-indexed** integer array `nums` consisting of `3 * n` elements.

You are allowed to remove any **subsequence** of elements of size **exactly** `n` from `nums`. The remaining `2 * n` elements will be divided into two **equal** parts:
* The first `n` elements belonging to the first part and their sum is <code>sum<sub>first</sub></code>.
* The next `n` elements belonging to the second part and their sum is <code>sum<sub>second</sub></code>.

The **difference in sums** of the two parts is denoted as <code>sum<sub>first</sub> - sum<sub>second</sub></code>.

* For example, if <code>sum<sub>first</sub> = 3</code> and <code>sum<sub>second</sub> = 2</code>, their difference is `1`.
* Similarly, if <code>sum<sub>first</sub> = 2</code> and <code>sum<sub>second</sub> = 3</code>, their difference is `-1`.

Return *the **minimum difference** possible between the sums of the two parts after the removal of* `n` *elements*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [3,1,2]
<strong>Output:</strong> -1
<strong>Explanation:</strong> Here, nums has 3 elements, so n = 1.
Thus we have to remove 1 element from nums and divide the array into two equal parts.
- If we remove nums[0] = 3, the array will be [1,2]. The difference in sums of the two parts will be 1 - 2 = -1.
- If we remove nums[1] = 1, the array will be [3,2]. The difference in sums of the two parts will be 3 - 2 = 1.
- If we remove nums[2] = 2, the array will be [3,1]. The difference in sums of the two parts will be 3 - 1 = 2.
The minimum difference between sums of the two parts is min(-1,1,2) = -1.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [7,9,5,8,1,3]
<strong>Output:</strong> 1
<strong>Explanation:</strong> Here n = 2. So we must remove 2 elements and divide the remaining array into two parts containing two elements each.
If we remove nums[2] = 5 and nums[3] = 8, the resultant array will be [7,9,1,3]. The difference in sums will be (7+9) - (1+3) = 12.
To obtain the minimum difference, we should remove nums[1] = 9 and nums[4] = 1. The resultant array becomes [7,5,8,3]. The difference in sums of the two parts is (7+5) - (8+3) = 1.
It can be shown that it is not possible to obtain a difference smaller than 1.
</pre>

#### Constraints:
* `nums.length == 3 * n`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>5</sup></code>

## Solutions (Rust)

### 1. Solution
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
