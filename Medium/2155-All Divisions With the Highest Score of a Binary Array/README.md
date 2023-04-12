# 2155. All Divisions With the Highest Score of a Binary Array
You are given a **0-indexed** binary array `nums` of length `n`. `nums` can be divided at index `i` (where `0 <= i <= n`) into two arrays (possibly empty) <code>nums<sub>left</sub></code> and <code>nums<sub>right</sub></code>:

* <code>nums<sub>left</sub></code> has all the elements of `nums` between index `0` and `i - 1` (**inclusive**), while <code>nums<sub>right</sub></code> has all the elements of nums between index `i` and `n - 1` (**inclusive**).
* If `i == 0`, <code>nums<sub>left</sub></code> is **empty**, while <code>nums<sub>right</sub></code> has all the elements of nums.
* If `i == n`, <code>nums<sub>left</sub></code> has all the elements of nums, while <code>nums<sub>right</sub></code> is **empty**.

The **division score** of an index `i` is the **sum** of the number of `0`'s in <code>nums<sub>left</sub></code> and the number of `1`'s in <code>nums<sub>right</sub></code>.

Return ***all distinct indices** that have the **highest** possible **division score***. You may return the answer in **any order**.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [0,0,1,0]
<strong>Output:</strong> [2,4]
<strong>Explanation:</strong> Division at index
- 0: numsleft is []. numsright is [0,0,1,0]. The score is 0 + 1 = 1.
- 1: numsleft is [0]. numsright is [0,1,0]. The score is 1 + 1 = 2.
- 2: numsleft is [0,0]. numsright is [1,0]. The score is 2 + 1 = 3.
- 3: numsleft is [0,0,1]. numsright is [0]. The score is 2 + 0 = 2.
- 4: numsleft is [0,0,1,0]. numsright is []. The score is 3 + 0 = 3.
Indices 2 and 4 both have the highest possible division score 3.
Note the answer [4,2] would also be accepted.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [0,0,0]
<strong>Output:</strong> [3]
<strong>Explanation:</strong> Division at index
- 0: numsleft is []. numsright is [0,0,0]. The score is 0 + 0 = 0.
- 1: numsleft is [0]. numsright is [0,0]. The score is 1 + 0 = 1.
- 2: numsleft is [0,0]. numsright is [0]. The score is 2 + 0 = 2.
- 3: numsleft is [0,0,0]. numsright is []. The score is 3 + 0 = 3.
Only index 3 has the highest possible division score 3.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,1]
<strong>Output:</strong> [0]
<strong>Explanation:</strong> Division at index
- 0: numsleft is []. numsright is [1,1]. The score is 0 + 2 = 2.
- 1: numsleft is [1]. numsright is [1]. The score is 0 + 1 = 1.
- 2: numsleft is [1,1]. numsright is []. The score is 0 + 0 = 0.
Only index 0 has the highest possible division score 2.
</pre>

#### Constraints:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* `nums[i]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn max_score_indices(nums: Vec<i32>) -> Vec<i32> {
        let mut left0 = 0;
        let mut right1 = nums.iter().filter(|&&x| x == 1).count();
        let mut max_score = right1;
        let mut ret = vec![0];

        for i in 0..nums.len() {
            match nums[i] {
                0 => left0 += 1,
                _ => right1 -= 1,
            }

            if left0 + right1 > max_score {
                max_score = left0 + right1;
                ret = vec![i as i32 + 1];
            } else if left0 + right1 == max_score {
                ret.push(i as i32 + 1);
            }
        }

        ret
    }
}
```
