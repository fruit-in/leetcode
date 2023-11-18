# 239. 滑动窗口最大值
给你一个整数数组 `nums`，有一个大小为 `k` 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 `k` 个数字。滑动窗口每次只向右移动一位。

返回 *滑动窗口中的最大值* 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,3,-1,-3,5,3,6,7], k = 3
<strong>输出:</strong> [3,3,5,5,6,7]
<strong>解释:</strong>
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
 1 [3  -1  -3] 5  3  6  7       3
 1  3 [-1  -3  5] 3  6  7       5
 1  3  -1 [-3  5  3] 6  7       5
 1  3  -1  -3 [5  3  6] 7       6
 1  3  -1  -3  5 [3  6  7]      7
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1], k = 1
<strong>输出:</strong> [1]
</pre>

#### 提示:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>-10<sup>4</sup> <= nums[i] <= 10<sup>4</sup></code>
* `1 <= k <= nums.length`

## 题解 (Rust)

### 1. 题解
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
