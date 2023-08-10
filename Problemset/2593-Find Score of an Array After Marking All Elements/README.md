# 2593. Find Score of an Array After Marking All Elements
You are given an array `nums` consisting of positive integers.

Starting with `score = 0`, apply the following algorithm:

* Choose the smallest integer of the array that is not marked. If there is a tie, choose the one with the smallest index.
* Add the value of the chosen integer to `score`.
* Mark **the chosen element and its two adjacent elements if they exist**.
* Repeat until all the array elements are marked.

Return *the score you get after applying the above algorithm*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [2,1,3,4,5,2]
<strong>Output:</strong> 7
<strong>Explanation:</strong> We mark the elements as follows:
- 1 is the smallest unmarked element, so we mark it and its two adjacent elements: [2,1,3,4,5,2].
- 2 is the smallest unmarked element, so we mark it and its left adjacent element: [2,1,3,4,5,2].
- 4 is the only remaining unmarked element, so we mark it: [2,1,3,4,5,2].
Our score is 1 + 2 + 4 = 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [2,3,5,1,3,2]
<strong>Output:</strong> 5
<strong>Explanation:</strong> We mark the elements as follows:
- 1 is the smallest unmarked element, so we mark it and its two adjacent elements: [2,3,5,1,3,2].
- 2 is the smallest unmarked element, since there are two of them, we choose the left-most one, so we mark the one at index 0 and its right adjacent element: [2,3,5,1,3,2].
- 2 is the only remaining unmarked element, so we mark it: [2,3,5,1,3,2].
Our score is 1 + 2 + 2 = 5.
</pre>

#### Constraints:
* <code>1 <= nums.length <= 10<sup>5</sup></code>
* <code>1 <= nums[i] <= 10<sup>6</sup></code>

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        let mut heap = nums
            .iter()
            .enumerate()
            .map(|(i, x)| Reverse((x, i)))
            .collect::<BinaryHeap<_>>();
        let mut indices = HashSet::new();
        let mut score = 0;

        while indices.len() < nums.len() {
            let Reverse((x, i)) = heap.pop().unwrap();

            if !indices.contains(&i) {
                score += *x as i64;
                indices.insert(i);
                if i > 0 {
                    indices.insert(i - 1);
                }
                if i < nums.len() - 1 {
                    indices.insert(i + 1);
                }
            }
        }

        score
    }
}
```
