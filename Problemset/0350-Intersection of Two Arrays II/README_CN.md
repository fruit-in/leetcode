# 350. 两个数组的交集 II
给定两个数组，编写一个函数来计算它们的交集。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,2,2,1], nums2 = [2,2]
<strong>输出:</strong> [2,2]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [4,9,5], nums2 = [9,4,9,8,4]
<strong>输出:</strong> [4,9]
</pre>

#### 说明:
* 输出结果中每个元素出现的次数，应与元素在两个数组中出现的次数一致。
* 我们可以不考虑输出结果的顺序。

#### 进阶:
* 如果给定的数组已经排好序呢？你将如何优化你的算法？
* 如果 *nums1* 的大小比 *nums2* 小很多，哪种方法更优？
* 如果 *nums2* 的元素存储在磁盘上，磁盘内存是有限的，并且你不能一次加载所有的元素到内存中，你该怎么办？

## 题解 (Rust)

### 1. 哈希表
```Rust
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map1 = HashMap::new();
        for n in nums1 {
            *map1.entry(n).or_insert(0) += 1;
        }

        let mut ret = Vec::new();

        for n in nums2 {
            if let Some(x) = map1.get_mut(&n) {
                *x -= 1;
                if *x == 0 {
                    map1.remove(&n);
                } 
                ret.push(n);
            }
        }

        ret
    }
}
```

### 2. 排序
```Rust
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort_unstable();
        nums2.sort_unstable();

        let mut i = 0;
        let mut j = 0;
        let mut ret = Vec::new();

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                ret.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            }
        }

        ret
    }
}
```
