# 349. 两个数组的交集
给定两个数组，编写一个函数来计算它们的交集。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,2,1], nums2 = [2,2]
<strong>输出:</strong> [2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [4,9,5], nums2 = [9,4,9,8,4]
<strong>输出:</strong> [9,4]
</pre>

#### 说明:
* 输出结果中的每个元素一定是唯一的。
* 我们可以不考虑输出结果的顺序。

## 题解 (Rust)

### 1. 暴力法
```Rust
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::new();
        for n1 in &nums1 {
            if !ret.contains(n1) {
                for n2 in &nums2 {
                    if n2 == n1 {
                        ret.push(*n1);
                        break;
                    }
                }
            }
        }
        ret
    }
}
```

### 2. 集合
```Rust
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<_> = nums1.iter().collect();
        let set2: HashSet<_> = nums2.iter().collect();

        set1.intersection(&set2).map(|&&x| x).collect()
    }
}
```

### 3. 排序
```Rust
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else if Some(&nums1[i]) == ret.last() {
                i += 1;
                j += 1;
            } else {
                ret.push(nums1[i]);
            }
        }

        ret
    }
}
```
