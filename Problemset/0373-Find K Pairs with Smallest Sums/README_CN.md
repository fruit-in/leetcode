# 373. 查找和最小的 K 对数字
给定两个以 **非递减顺序排列** 的整数数组 `nums1` 和 `nums2` , 以及一个整数 `k` 。

定义一对值 `(u,v)`，其中第一个元素来自 `nums1`，第二个元素来自 `nums2` 。

请找到和最小的 `k` 个数对 <code>(u<sub>1</sub>,v<sub>1</sub>)</code>,  <code>(u<sub>2</sub>,v<sub>2</sub>)</code>  ...  <code>(u<sub>k</sub>,v<sub>k</sub>)</code> 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,7,11], nums2 = [2,4,6], k = 3
<strong>输出:</strong> [[1,2],[1,4],[1,6]]
<strong>解释:</strong> 返回序列中的前 3 对数：
     [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [1,1,2], nums2 = [1,2,3], k = 2
<strong>输出:</strong> [[1,1],[1,1]]
<strong>解释:</strong> 返回序列中的前 2 对数：
     [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
</pre>

#### 提示:
* <code>1 <= nums1.length, nums2.length <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums1[i], nums2[i] <= 10<sup>9</sup></code>
* `nums1` 和 `nums2` 均为 **升序排列**
* <code>1 <= k <= 10<sup>4</sup></code>
* `k <= nums1.length * nums2.length`

## 题解 (Rust)

### 1. 题解
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
