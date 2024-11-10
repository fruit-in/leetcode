# 2454. Next Greater Element IV
You are given a **0-indexed** array of non-negative integers `nums`. For each integer in `nums`, you must find its respective **second greater** integer.

The **second greater** integer of `nums[i]` is `nums[j]` such that:

* `j > i`
* `nums[j] > nums[i]`
* There exists **exactly one** index `k` such that `nums[k] > nums[i]` and `i < k < j`.

If there is no such `nums[j]`, the second greater integer is considered to be `-1`.

* For example, in the array `[1, 2, 4, 3]`, the second greater integer of `1` is `4`, `2` is `3`, and that of `3` and `4` is `-1`.

Return *an integer array* `answer`, *where* `answer[i]` *is the second greater integer of* `nums[i]`.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,4,0,9,6]
<strong>Output:</strong> [9,6,6,-1,-1]
<strong>Explanation:</strong>
0th index: 4 is the first integer greater than 2, and 9 is the second integer greater than 2, to the right of 2.
1st index: 9 is the first, and 6 is the second integer greater than 4, to the right of 4.
2nd index: 9 is the first, and 6 is the second integer greater than 0, to the right of 0.
3rd index: There is no integer greater than 9 to its right, so the second greater integer is considered to be -1.
4th index: There is no integer greater than 6 to its right, so the second greater integer is considered to be -1.
Thus, we return [9,6,6,-1,-1].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [3,3]
<strong>Output:</strong> [-1,-1]
<strong>Explanation:</strong>
We return [-1,-1] since neither integer has any integer greater than it.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>0 <= nums[i] <= 10<sup>9</sup></code>

## Solutions (Rust)

### 1. Solution
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
