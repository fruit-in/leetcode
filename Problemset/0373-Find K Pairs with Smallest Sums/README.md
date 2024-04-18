# 373. Find K Pairs with Smallest Sums
You are given two integer arrays `nums1` and `nums2` sorted in **non-decreasing order** and an integer `k`.

Define a pair `(u, v)` which consists of one element from the first array and one element from the second array.

Return *the* `k` *pairs* <code>(u<sub>1</sub>, v<sub>1</sub>), (u<sub>2</sub>, v<sub>2</sub>), ..., (u<sub>k</sub>, v<sub>k</sub>)</code> *with the smallest sums*.

#### Example 1:
<pre>
<strong>Input:</strong> nums1 = [1,7,11], nums2 = [2,4,6], k = 3
<strong>Output:</strong> [[1,2],[1,4],[1,6]]
<strong>Explanation:</strong> The first 3 pairs are returned from the sequence: [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> nums1 = [1,1,2], nums2 = [1,2,3], k = 2
<strong>Output:</strong> [[1,1],[1,1]]
<strong>Explanation:</strong> The first 2 pairs are returned from the sequence: [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
</pre>

#### Constraints:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums1[i], nums2[i] <= 10<sup>9</sup></code>
* `nums1` and `nums2` both are sorted in **non-decreasing order**.
* <code>1 <= k <= 10<sup>4</sup></code>
* `k <= nums1.length * nums2.length`

## Solutions (Rust)

### 1. Solution
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = (0..nums1.len().min(k as usize))
            .map(|i| (Reverse(nums1[i] + nums2[0]), i, 0))
            .collect::<BinaryHeap<_>>();
        let mut ret = vec![];

        for _ in 0..k {
            let (_, i, j) = heap.pop().unwrap();

            if j + 1 < nums2.len() {
                heap.push((Reverse(nums1[i] + nums2[j + 1]), i, j + 1));
            }
            ret.push(vec![nums1[i], nums2[j]]);
        }

        ret
    }
}
```
