# 268. Missing Number
Given an array containing *n* distinct numbers taken from ```0, 1, 2, ..., n```, find the one that is missing from the array.

#### Example 1:
<pre>
<strong>Input:</strong> [3,0,1]
<strong>Output:</strong> 2
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [9,6,4,2,3,5,7,0,1]
<strong>Output:</strong> 8
</pre>

#### Note:
Your algorithm should run in linear runtime complexity. Could you implement it using only constant extra space complexity?

## Solutions (Rust)

### 1. Gauss' Formula
```Rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n + 1) / 2 - sum
    }
}
```

### 2. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let nums: HashSet<_> = nums.iter().collect();
        for i in 0..nums.len() {
            if !nums.contains(&(i as i32)) {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}
```

### 3. XOR
```Rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut miss = 0;
        for i in 0..nums.len() {
            miss ^= nums[i] ^ (i as i32 + 1);
        }
        miss
    }
}
```

### 4. Sort
```Rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let m = (l + r) / 2;
            if nums[m] != m as i32 {
                if m == 0 || nums[m - 1] == m as i32 - 1 {
                    return m as i32;
                }
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        nums.len() as i32
    }
}
```
