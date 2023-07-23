# 136. Single Number
Given a **non-empty** array of integers, every element appears *twice* except for one. Find that single one.

#### Note:
Your algorithm should have a linear runtime complexity. Could you implement it without using extra memory?

#### Example 1:
<pre>
<strong>Input:</strong> [2,2,1]
<strong>Output:</strong> 1
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [4,1,2,1,2]
<strong>Output:</strong> 4
</pre>

## Solutions (Rust)

### 1. Set
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();
        for n in nums {
            if set.contains(&n) {
                set.remove(&n);
            } else {
                set.insert(n);
            }
        }
        *set.iter().next().unwrap()
    }
}
```

### 2. Mathematical
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.clone().drain(..).collect();
        let sum1: i32 = set.iter().sum();
        let sum2: i32 = nums.iter().sum();
        2 * sum1 - sum2
    }
}
```

### 3. XOR
```Rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut z = 0;
        for n in nums {
            z ^= n;
        }
        z
    }
}
```
