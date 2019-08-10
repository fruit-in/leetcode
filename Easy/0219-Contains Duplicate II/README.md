# 219. Contains Duplicate II
Given an array of integers and an integer *k*, find out whether there are two distinct indices *i* and *j* in the array such that **nums[i] = nums[j]** and the **absolute** difference between *i* and *j* is at most *k*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,1], k = 3
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,0,1,1], k = 1
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,2,3,1,2,3], k = 2
<strong>Output:</strong> false
</pre>

## Solutions (Rust)

### 1. Linear Scan
```Rust
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let len = nums.len();
        for i in 0..len {
            let mut j = i + 1;
            while j <= i + k as usize && j < len {
                if nums[i] == nums[j] {
                    return true;
                }
                j += 1;
            }
        }
        false
    }
}
```

### 2. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) && i <= map[&nums[i]] + k {
                return true;
            } else {
                map.insert(nums[i], i);
            }
        }
        false
    }
}
```

### 3. Sort
```Rust
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut v = Vec::new();
        for i in 0..nums.len() {
            v.push((nums[i], i));
        }
        v.sort_unstable();
        for i in 1..nums.len() {
            if v[i - 1].0 == v[i].0 && v[i].1 <= v[i - 1].1 + k {
                return true;
            }
        }
        false
    }
}
```
