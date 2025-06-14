# 2542. 最大子序列的分数
给你两个下标从 **0** 开始的整数数组 `nums1` 和 `nums2` ，两者长度都是 `n` ，再给你一个正整数 `k` 。你必须从 `nums1` 中选一个长度为 `k` 的 **子序列** 对应的下标。

对于选择的下标 <code>i<sup>0</sup></code> ，<code>i<sup>1</sup></code> ，...， <code>i<sup>k - 1</sup></code> ，你的 **分数** 定义如下：
* `nums1` 中下标对应元素求和，乘以 `nums2` 中下标对应元素的 **最小值** 。
* 用公式表示： <code>(nums1[i<sup>0</sup>] + nums1[i<sup>1</sup>] +...+ nums1[i<sup>k - 1</sup>]) * min(nums2[i<sup>0</sup>] , nums2[i<sup>1</sup>], ... ,nums2[i<sup>k - 1</sup>])</code> 。

请你返回 **最大** 可能的分数。

一个数组的 **子序列** 下标是集合 `{0, 1, ..., n-1}` 中删除若干元素得到的剩余集合，也可以不删除任何元素。

#### 示例 1:
<pre>
<strong>输入:</strong> nums1 = [1,3,3,2], nums2 = [2,1,3,4], k = 3
<strong>输出:</strong> 12
<strong>解释:</strong>
四个可能的子序列分数为：
- 选择下标 0 ，1 和 2 ，得到分数 (1+3+3) * min(2,1,3) = 7 。
- 选择下标 0 ，1 和 3 ，得到分数 (1+3+2) * min(2,1,4) = 6 。
- 选择下标 0 ，2 和 3 ，得到分数 (1+3+2) * min(2,3,4) = 12 。
- 选择下标 1 ，2 和 3 ，得到分数 (3+3+2) * min(1,3,4) = 8 。
所以最大分数为 12 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums1 = [4,2,3,1,1], nums2 = [7,5,10,9,6], k = 1
<strong>输出:</strong> 30
<strong>解释:</strong>
选择下标 2 最优：nums1[2] * nums2[2] = 3 * 10 = 30 是最大可能分数。
</pre>

#### 提示:
* `n == nums1.length == nums2.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>0 <= nums1[i], nums2[j] <= 10<sup>5</sup></code>
* `1 <= k <= n`

## 题解 (Rust)

### 1. 题解
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut nums = nums1.iter().zip(nums2.iter()).collect::<Vec<_>>();
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        let mut ret = 0;

        nums.sort_unstable_by_key(|&(_, y)| -y);

        for i in 0..nums.len() {
            heap.push(-nums[i].0);
            sum += *nums[i].0 as i64;

            if i >= k - 1 {
                if heap.len() > k {
                    sum -= -heap.pop().unwrap() as i64;
                }
                ret = ret.max(sum * *nums[i].1 as i64);
            }
        }

        ret
    }
}
```
