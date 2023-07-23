# 217. Contains Duplicate
Given an array of integers, find if the array contains any duplicates.

Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,1]
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [1,2,3,4]
<strong>Output:</strong> false
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> [1,1,1,3,3,4,3,2,4,2]
<strong>Output:</strong> true
</pre>

## Solutions (Rust)

### 1. Set-len
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }
}
```

### 2. Linear Scan
```Rust
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}
```

### 3. Set-contains
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                return true;
            } else {
                set.insert(n);
            }
        }
        false
    }
}
```

### 4. Sort
```Rust
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i - 1] == nums[i] {
                return true;
            }
        }
        false
    }
}
```
