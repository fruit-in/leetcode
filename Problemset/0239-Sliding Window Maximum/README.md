# 239. Sliding Window Maximum
You are given an array of integers `nums`, there is a sliding window of size `k` which is moving from the very left of the array to the very right. You can only see the `k` numbers in the window. Each time the sliding window moves right by one position.

Return *the max sliding window*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,3,-1,-3,5,3,6,7], k = 3
<strong>Output:</strong> [3,3,5,5,6,7]
<strong>Explanation:</strong>
Window position                Max
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1], k = 1
<strong>Output:</strong> [1]
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>
* `1 <= k <= nums.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut ret = vec![];

        for i in 0..nums.len() {
            if i >= k && *deque.front().unwrap_or(&100000) <= i - k {
                deque.pop_front();
            }

            while let Some(&j) = deque.back() {
                if nums[j] < nums[i] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(i);

            if i >= k - 1 {
                ret.push(nums[*deque.front().unwrap()]);
            }
        }

        ret
    }
}
```
