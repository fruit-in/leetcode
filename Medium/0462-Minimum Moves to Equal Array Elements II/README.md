# 462. Minimum Moves to Equal Array Elements II
Given a **non-empty** integer array, find the minimum number of moves required to make all array elements equal, where a move is incrementing a selected element by 1 or decrementing a selected element by 1.

You may assume the array's length is at most 10,000.

#### Example:
<pre>
<strong>Input:</strong>
[1,2,3]
<strong>Output:</strong>
2
<strong>Explanation:</strong>
Only two moves are needed (remember each move increments or decrements one element):

[1,2,3]  =>  [2,2,3]  =>  [2,2,2]
</pre>

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mid = nums[nums.len() / 2];

        nums.into_iter().map(|x| (x - mid).abs()).sum()
    }
}
```
