# 217. 存在重复元素
给定一个整数数组，判断是否存在重复元素。

如果任何值在数组中出现至少两次，函数返回 true。如果数组中每个元素都不相同，则返回 false。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,1]
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1,2,3,4]
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1,1,1,3,3,4,3,2,4,2]
<strong>输出:</strong> true
</pre>

## 题解 (Rust)

### 1. 集合-比较长度
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() > nums.iter().collect::<HashSet<_>>().len()
    }
}
```

### 2. 线性扫描
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

### 3. 集合-是否已存在
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

### 4. 排序
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
