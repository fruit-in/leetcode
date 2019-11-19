# 561. Array Partition I
Given an array of **2n** integers, your task is to group these integers into **n** pairs of integer, say (a<sub>1</sub>, b<sub>1</sub>), (a<sub>2</sub>, b<sub>2</sub>), ..., (a<sub>n</sub>, b<sub>n</sub>) which makes sum of min(a<sub>i</sub>, b<sub>i</sub>) for all i from 1 to n as large as possible.

#### Example 1:
<pre>
<strong>Input:</strong> [1,4,3,2]
<strong>Output:</strong> 4
<strong>Explanation:</strong> n is 2, and the maximum sum of pairs is 4 = min(1, 2) + min(3, 4).
</pre>

#### Note:
1. **n** is a positive integer, which is in the range of [1, 10000].
2. All the integers in the array will be in the range of [-10000, 10000].

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        nums.iter().step_by(2).sum()
    }
}
```
