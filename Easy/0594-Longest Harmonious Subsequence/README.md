# 594. Longest Harmonious Subsequence
We define a harmounious array as an array where the difference between its maximum value and its minimum value is **exactly** 1.

Now, given an integer array, you need to find the length of its longest harmonious subsequence among all its possible [subsequences](https://en.wikipedia.org/wiki/Subsequence).

#### Example 1:
<pre>
<strong>Input:</strong> [1,3,2,2,5,2,3,7]
<strong>Output:</strong> 5
<strong>Explanation:</strong> The longest harmonious subsequence is [3,2,2,2,3].
</pre>

**Note:** The length of the input array will not exceed 20,000.

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        for i in 0..nums.len() {
            if nums.contains(&(nums[i] + 1)) {
                let mut len = 0;

                for j in 0..nums.len() {
                    if nums[j] == nums[i] || nums[j] == nums[i] + 1 {
                        len += 1;
                    }
                }

                max_len = max_len.max(len);
            }
        }

        max_len as i32
    }
}
```

### 2. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        let mut max_len = 0;

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        for (num, cnt1) in counter.iter() {
            if let Some(&cnt2) = counter.get(&(num + 1)) {
                max_len = max_len.max(cnt1 + cnt2);
            }
        }

        max_len
    }
}
```

### 3. Sort
```Rust
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max_len = 0;

        let mut i = 0;
        let mut j = 1;
        while i < nums.len() {
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }

            let k = j;

            if j < nums.len() && nums[j] == nums[i] + 1 {
                while j < nums.len() && nums[j] == nums[i] + 1 {
                    j += 1;
                }

                max_len = max_len.max(j - i);
            }

            i = k;
        }

        max_len as i32
    }
}
```
