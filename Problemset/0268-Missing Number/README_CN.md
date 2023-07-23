# 268. 缺失数字
给定一个包含 ```0, 1, 2, ..., n``` 中 *n* 个数的序列，找出 0 .. *n* 中没有出现在序列中的那个数。

#### 示例 1:
<pre>
<strong>输入:</strong> [3,0,1]
<strong>输出:</strong> 2
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [9,6,4,2,3,5,7,0,1]
<strong>输出:</strong> 8
</pre>

#### 说明:
你的算法应具有线性时间复杂度。你能否仅使用额外常数空间来实现?

## 题解 (Rust)

### 1. 高斯公式
```Rust
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let sum: i32 = nums.iter().sum();
        n * (n + 1) / 2 - sum
    }
}
```

### 2. 集合
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

### 3. 异或
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

### 4. 排序
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
