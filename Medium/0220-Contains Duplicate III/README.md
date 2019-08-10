# 220. Contains Duplicate III
Given an array of integers, find out whether there are two distinct indices *i* and *j* in the array such that the **absolute** difference between **nums[i]** and **nums[j]** is at most *t* and the **absolute** difference between *i* and *j* is at most *k*.

#### Example 1:
<pre>
<strong>Input:</strong> nums = [1,2,3,1], k = 3, t = 0
<strong>Output:</strong> true
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums = [1,0,1,1], k = 1, t = 2
<strong>Output:</strong> true
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> nums = [1,5,9,1,5,9], k = 2, t = 3
<strong>Output:</strong> false
</pre>

## Solutions (Rust)

### 1. Linear Scan-k first
```Rust
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let k = k as usize;
        let t = t as i64;
        let len = nums.len();
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        for i in 0..len {
            for n in &nums[(i + 1)..len.min(i + 1 + k)] {
                if (nums[i] - n).abs() <= t {
                    return true;
                }
            }
        }
        false
    }
}
```

### 2. Linear Scan-t first
```Rust
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let mut len = nums.len();
        let mut v = Vec::new();
        for i in 0..len {
            v.push((nums[i] as i64, i as i32));
        }
        v.sort_unstable();
        for i in 0..len {
            let mut j = i + 1;
            while j < len && v[j].0 - v[i].0 <= t as i64 {
                if (v[j].1 - v[i].1).abs() <= k {
                    return true;
                }
                j += 1;
            }
        }
        false
    }
}
```

### 3. TreeMap
```Rust
use std::collections::BTreeSet;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let mut tree = BTreeSet::new();
        for i in 0..nums.len() {
            let n = nums[i];
            if tree.range((n - t)..=(n + t)).count() > 0 {
                return true;
            }
            tree.insert(n);
            if i >= k {
                tree.remove(&nums[i - k]);
            }
        }
        false
    }
}
```

### 4. Bucket
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        if t < 0 {
            return false;
        }
        let k = k as usize;
        let t = t as i64;
        let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
        let mut buckets = HashMap::new();
        for i in 0..nums.len() {
            let bucket_id = (nums[i] as f64 / (t + 1) as f64).floor() as i64;
            if buckets.insert(bucket_id, nums[i]) != None {
                return true;
            }
            match buckets.get(&(bucket_id - 1)) {
                Some(n) => {
                    if (n - nums[i]).abs() <= t {
                        return true;
                    }
                },
                None => (),
            }
            match buckets.get(&(bucket_id + 1)) {
                Some(n) => {
                    if (n - nums[i]).abs() <= t {
                        return true;
                    }
                },
                None => (),
            }
            if i >= k {
                buckets.remove(&(nums[i - k] / (t + 1)));
            }
        }
        false
    }
}
```
