# 136. 只出现一次的数字
给定一个**非空**整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

#### 说明:
你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？

#### 示例 1:
<pre>
<strong>输入:</strong> [2,2,1]
<strong>输出:</strong> 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [4,1,2,1,2]
<strong>输出:</strong> 4
</pre>

## 题解 (Rust)

### 1. 集合
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

### 2. 数学
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

### 3. 异或
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
