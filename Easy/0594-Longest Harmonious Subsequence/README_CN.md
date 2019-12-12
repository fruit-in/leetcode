# 594. 最长和谐子序列
和谐数组是指一个数组里元素的最大值和最小值之间的差别正好是1。

现在，给定一个整数数组，你需要在所有可能的子序列中找到最长的和谐子序列的长度。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,3,2,2,5,2,3,7]
<strong>输出:</strong> 5
<strong>原因:</strong> 最长的和谐数组是：[3,2,2,2,3].
</pre>

**说明:** 输入的数组长度最大不超过20,000.

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;

        for i in 0..nums.len() {
            if nums.contains(&(nums[i] + 1)) {
                let mut len = 0;

                for j in 0..nums.len() {
                    if nums[j] == nums[i] || nums[j] == nums[i] + 1 {
                        len += 1;
                    }
                }

                max_len = max_len.max(len);
            }
        }

        max_len as i32
    }
}
```

### 2. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        let mut max_len = 0;

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        for (num, cnt1) in counter.iter() {
            if let Some(&cnt2) = counter.get(&(num + 1)) {
                max_len = max_len.max(cnt1 + cnt2);
            }
        }

        max_len
    }
}
```

### 3. 排序
```Rust
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut max_len = 0;

        let mut i = 0;
        let mut j = 1;
        while i < nums.len() {
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }

            let k = j;

            if j < nums.len() && nums[j] == nums[i] + 1 {
                while j < nums.len() && nums[j] == nums[i] + 1 {
                    j += 1;
                }

                max_len = max_len.max(j - i);
            }

            i = k;
        }

        max_len as i32
    }
}
```
