# 786. 第 K 个最小的质数分数
给你一个按递增顺序排序的数组 `arr` 和一个整数 `k` 。数组 `arr` 由 `1` 和若干 **质数** 组成，且其中所有整数互不相同。

对于每对满足 `0 <= i < j < arr.length` 的 `i` 和 `j` ，可以得到分数 `arr[i] / arr[j]` 。

那么第 `k` 个最小的分数是多少呢?  以长度为 `2` 的整数数组返回你的答案, 这里 `answer[0] == arr[i]` 且 `answer[1] == arr[j]` 。

#### 示例 1:
<pre>
<strong>输入:</strong> arr = [1,2,3,5], k = 3
<strong>输出:</strong> [2,5]
<strong>解释:</strong> 已构造好的分数,排序后如下所示:
1/5, 1/3, 2/5, 1/2, 3/5, 2/3
很明显第三个最小的分数是 2/5
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> arr = [1,7], k = 1
<strong>输出:</strong> [1,7]
</pre>

#### 提示:
* `2 <= arr.length <= 1000`
* <code>1 <= arr[i] <= 3 * 10<sup>4</sup></code>
* `arr[0] == 1`
* `arr[i]` 是一个 **质数** ，`i > 0`
* `arr` 中的所有数字 **互不相同** ，且按 **严格递增** 排序
* `1 <= k <= arr.length * (arr.length - 1) / 2`

**进阶：**你可以设计并实现时间复杂度小于 <code>O(n<sup>2</sup>)</code> 的算法解决此问题吗？

## 题解 (Rust)

### 1. 题解
```Rust
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Fraction(i32, i32);

impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = self.0 as i64 * other.1 as i64;
        let y = self.1 as i64 * other.0 as i64;

        y.cmp(&x)
    }
}

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut heap = BinaryHeap::new();

        for i in 0..arr.len() - 1 {
            heap.push((Fraction(arr[i], arr[arr.len() - 1]), i, arr.len() - 1));
        }

        for _ in 0..k - 1 {
            let (_, i, j) = heap.pop().unwrap();

            if i < j - 1 {
                heap.push((Fraction(arr[i], arr[j - 1]), i, j - 1));
            }
        }

        let (_, i, j) = heap.pop().unwrap();

        vec![arr[i], arr[j]]
    }
}
```
