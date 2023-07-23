# 532. K-diff Pairs in an Array
Given an array of integers and an integer **k**, you need to find the number of **unique** k-diff pairs in the array. Here a **k-diff** pair is defined as an integer pair (i, j), where **i** and **j** are both numbers in the array and their [absolute difference](https://en.wikipedia.org/wiki/Absolute_difference) is **k**.

#### Example 1:
<pre>
<strong>Input:</strong> [3, 1, 4, 1, 5], k = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are two 2-diff pairs in the array, (1, 3) and (3, 5).
Although we have two 1s in the input, we should only return the number of <strong>unique</strong> pairs.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1, 2, 3, 4, 5], k = 1
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1, 3, 1, 5, 4], k = 0
<strong>Output:</strong> 1
<strong>Explanation:</strong> There is one 0-diff pair in the array, (1, 1).
</pre>

#### Note:
1. The pairs (i, j) and (j, i) count as the same pair.
2. The length of the array won't exceed 10,000.
3. All the integers in the given input belong to the range: [-1e7, 1e7].

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut nums_set = HashSet::new();
        let mut pairs_j = HashSet::new();

        for n in nums {
            if nums_set.contains(&(n - k)) {
                pairs_j.insert(n);
            }
            if nums_set.contains(&(n + k)) {
                pairs_j.insert(n + k);
            }

            nums_set.insert(n);
        }

        pairs_j.len() as i32
    }
}
```

### 2. Binary Search
```Rust
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i + 1..].binary_search(&(nums[i] + k)).is_ok() {
                ret += 1;
            }
        }

        ret
    }
}
```
