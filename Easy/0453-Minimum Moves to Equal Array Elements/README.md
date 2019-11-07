# 453. Minimum Moves to Equal Array Elements
Given a **non-empty** integer array of size *n*, find the minimum number of moves required to make all array elements equal, where a move is incrementing *n* - 1 elements by 1.

#### Example:
<pre>
<strong>Input:</strong>
[1,2,3]
<strong>Output:</strong>
3
<strong>Explanation:</strong>
Only three moves are needed (remember each move increments two elements):

[1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
</pre>

## Solutions (Rust)

### 1. Sort
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        let mut ret = 0;

        for i in 1..nums.len() {
            ret += (nums[i - 1] - nums[i]) * i as i32;
        }

        ret
    }
}
```

### 2. Mathematical
```Rust
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min_num = nums.iter().min().unwrap();

        nums.iter().map(|&x| x - min_num).sum()
    }
}
```
