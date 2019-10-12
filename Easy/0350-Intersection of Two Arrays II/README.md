# 350. Intersection of Two Arrays II
Given two arrays, write a function to compute their intersection.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,2,2,1], nums2 = [2,2]
<strong>Output:</strong> [2,2]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [4,9,5], nums2 = [9,4,9,8,4]
<strong>Output:</strong> [4,9]
</pre>

#### Note:
* Each element in the result should appear as many times as it shows in both arrays.
* The result can be in any order.

#### Follow up:
* What if the given array is already sorted? How would you optimize your algorithm?
* What if *nums1*'s size is small compared to *nums2*'s size? Which algorithm is better?
* What if elements of *nums2* are stored on disk, and the memory is limited such that you cannot load all elements into the memory at once?

## Solutions (Rust)

### 1. HashMap
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

### 2. Sort
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
