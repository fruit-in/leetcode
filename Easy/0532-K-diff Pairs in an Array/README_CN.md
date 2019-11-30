# 532. 数组中的K-diff数对
给定一个整数数组和一个整数 **k**, 你需要在数组里找到**不同的** k-diff 数对。这里将 **k-diff** 数对定义为一个整数对 (i, j), 其中 **i** 和 **j** 都是数组中的数字，且两数之差的绝对值是 **k**.

#### 示例 1:
<pre>
<strong>输入:</strong> [3, 1, 4, 1, 5], k = 2
<strong>输出:</strong> 2
<strong>解释:</strong> 数组中有两个 2-diff 数对, (1, 3) 和 (3, 5)。
尽管数组中有两个1，但我们只应返回不同的数对的数量。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [1, 2, 3, 4, 5], k = 1
<strong>输出:</strong> 4
<strong>解释:</strong> 数组中有四个 1-diff 数对, (1, 2), (2, 3), (3, 4) 和 (4, 5)。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> [1, 3, 1, 5, 4], k = 0
<strong>输出:</strong> 1
<strong>解释:</strong> 数组中只有一个 0-diff 数对，(1, 1)。
</pre>

#### 注意:
1. 数对 (i, j) 和数对 (j, i) 被算作同一数对。
2. 数组的长度不超过10,000。
3. 所有输入的整数的范围在 [-1e7, 1e7]。

## 题解 (Rust)

### 1. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        if k < 0 {
            return 0;
        }

        let mut nums_set = HashSet::new();
        let mut pairs_j = HashSet::new();

        for n in nums {
            if nums_set.contains(&(n - k)) {
                pairs_j.insert(n);
            }
            if nums_set.contains(&(n + k)) {
                pairs_j.insert(n + k);
            }

            nums_set.insert(n);
        }

        pairs_j.len() as i32
    }
}
```

### 2. 二分查找
```Rust
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ret = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i + 1..].binary_search(&(nums[i] + k)).is_ok() {
                ret += 1;
            }
        }

        ret
    }
}
```
