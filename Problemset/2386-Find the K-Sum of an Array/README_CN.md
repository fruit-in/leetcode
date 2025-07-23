# 2386. 找出数组的第 K 大和
给你一个整数数组 `nums` 和一个 **正** 整数 `k` 。你可以选择数组的任一 **子序列** 并且对其全部元素求和。

数组的 **第 k 大和** 定义为：可以获得的第 `k` 个 **最大** 子序列和（子序列和允许出现重复）

返回数组的 **第 k 大和** 。

子序列是一个可以由其他数组删除某些或不删除元素派生而来的数组，且派生过程不改变剩余元素的顺序。

**注意：**空子序列的和视作 `0` 。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,4,-2], k = 5
<strong>输出:</strong> 2
<strong>解释:</strong> 所有可能获得的子序列和列出如下，按递减顺序排列：
6、4、4、2、2、0、0、-2
数组的第 5 大和是 2 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,-2,3,4,-10,12], k = 16
<strong>输出:</strong> 10
<strong>解释:</strong> 数组的第 16 大和是 10 。
</pre>

#### 提示:
* `n == nums.length`
* <code>1 <= n <= 10<sup>5</sup></code>
* <code>-10<sup>9</sup> <= nums[i] <= 10<sup>9</sup></code>
* <code>1 <= k <= min(2000, 2<sup>n</sup>)</code>

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut max_sum = nums.iter().map(|&x| x.max(0) as i64).sum::<i64>();
        let mut max_heap = BinaryHeap::from([max_sum]);
        let mut min_heap = BinaryHeap::<Reverse<i64>>::new();
        let mut sorted_abs_nums = nums.iter().map(|&x| x.abs() as i64).collect::<Vec<_>>();
        sorted_abs_nums.sort_unstable();

        for i in 0..sorted_abs_nums.len() {
            while let Some(x) = max_heap.pop() {
                if min_heap.len() == k && x <= min_heap.peek().unwrap().0 {
                    break;
                }
                min_heap.push(Reverse(x));
                if min_heap.len() < k || x - sorted_abs_nums[i] > min_heap.peek().unwrap().0 {
                    min_heap.push(Reverse(x - sorted_abs_nums[i]));
                }
                while min_heap.len() > k {
                    min_heap.pop();
                }
            }

            if min_heap.len() == k
                && (i == sorted_abs_nums.len() - 1
                    || max_sum - sorted_abs_nums[i + 1] <= min_heap.peek().unwrap().0)
            {
                return min_heap.pop().unwrap().0;
            }

            max_heap = BinaryHeap::new();

            while let Some(Reverse(x)) = min_heap.pop() {
                max_heap.push(x);
            }
        }

        unreachable!()
    }
}
```
