# 169. Majority Element
Given an array of size *n*, find the majority element. The majority element is the element that appears **more than** ```⌊ n/2 ⌋``` times.

You may assume that the array is non-empty and the majority element always exist in the array.

#### Example 1:
<pre>
<strong>Input:</strong> [3,2,3]
<strong>Output:</strong> 3
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [2,2,1,1,1,2,2]
<strong>Output:</strong> 2
</pre>

## Solutions (Rust)

### 1. Brute Force
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let mut cnt = 0;
            for j in i..nums.len() {
                if nums[j] == nums[i] {
                    cnt += 1;
                }
                if cnt > nums.len() / 2 {
                    return nums[i];
                }
            }
        }
        0
    }
}
```

### 2. HashMap
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for &n in &nums {
            let cnt = map.entry(n).or_insert(0);
            *cnt += 1;
            if *cnt > nums.len() / 2 {
                return n;
            }
        }
        0
    }
}
```

### 3. Sort
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}
```

### 4. Divide & Conquer 
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let half = nums.len() / 2;
        let major_l = Self::majority_element(nums[..half].to_vec());
        let major_r = Self::majority_element(nums[half..].to_vec());

        if major_l == major_r {
            return major_l;
        }

        let mut cnt_l = 0;
        let mut cnt_r = 0;
        for &n in &nums {
            if n == major_l {
                cnt_l += 1;
                if cnt_l > half {
                    return major_l;
                }
            }
            if n == major_r {
                cnt_r += 1;
                if cnt_r > half {
                    return major_r;
                }
            }
        }
        0
    }
}
```

### 5. Boyer-Moore Voting Algorithm
```Rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut cnt = 1;
        let mut major = nums[0];
        for &n in &nums[1..] {
            if cnt == 0 {
                major = n;
            }
            if n == major {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        }
        major
    }
}
```
