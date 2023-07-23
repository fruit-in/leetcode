# 4. Median of Two Sorted Arrays
There are two sorted arrays **nums1** and **nums2** of size m and n respectively.

Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).

You may assume **nums1** and **nums2** cannot be both empty.

#### Example 1:
```
nums1 = [1, 3]
nums2 = [2]

The median is 2.0
```

#### Example 2:
```
nums1 = [1, 2]
nums2 = [3, 4]

The median is (2 + 3)/2 = 2.5
```

## Solutions (Rust)

### 1. Solution
```Rust
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        if nums1.len() > nums2.len() {
            let tmp = nums1;
            nums1 = nums2;
            nums2 = tmp;
        }
        let m = nums1.len();
        let n = nums2.len();
        let mut l = 0;
        let mut r = m;

        while l <= r {
            let i = (l + r) / 2;
            let j = (m + n) / 2 - i;

            if i < m && nums1[i] < nums2[j - 1] {
                l = i + 1;
            } else if i > 0 && nums1[i - 1] > nums2[j] {
                r = i - 1;
            } else {
                let r_min = if i == m {
                    nums2[j]
                } else if j == n {
                    nums1[i]
                } else {
                    nums1[i].min(nums2[j])
                };

                if (m + n) % 2 == 1 {
                    return r_min as f64;
                }

                let l_max = if i == 0 {
                    nums2[j - 1]
                } else if j == 0 {
                    nums1[i - 1]
                } else {
                    nums1[i - 1].max(nums2[j - 1])
                };

                return (l_max + r_min) as f64 / 2.;
            }
        }

        0.
    }
}
```
