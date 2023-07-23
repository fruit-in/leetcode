# 219. 存在重复元素 II
给定一个整数数组和一个整数 *k*，判断数组中是否存在两个不同的索引 *i* 和 *j*，使得 **nums [i] = nums [j]**，并且 *i* 和 *j* 的差的绝对值最大为 *k*。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,2,3,1], k = 3
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,0,1,1], k = 1
<strong>输出:</strong> true
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [1,2,3,1,2,3], k = 2
<strong>输出:</strong> false
</pre>

## 题解 (Rust)

### 1. 线性扫描
```Rust
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let len = nums.len();
        for i in 0..len {
            for n in &nums[(i + 1)..len.min(i + 1 + k)] {
                if nums[i] - n == 0 {
                    return true;
                }
            }
        }
        false
    }
}
```

### 2. 哈希表
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

### 3. 排序
```Rust
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut v = Vec::new();
        for i in 0..nums.len() {
            v.push((nums[i], i));
        }
        v.sort_unstable();
        for i in 1..v.len() {
            if v[i - 1].0 == v[i].0 && v[i].1 <= v[i - 1].1 + k {
                return true;
            }
        }
        false
    }
}
```
