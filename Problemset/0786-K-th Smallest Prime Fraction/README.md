# 786. K-th Smallest Prime Fraction
You are given a sorted integer array `arr` containing `1` and **prime** numbers, where all the integers of `arr` are unique. You are also given an integer `k`.

For every `i` and `j` where `0 <= i < j < arr.length`, we consider the fraction `arr[i] / arr[j]`.

Return *the* <code>k<sup>th</sup></code> *smallest fraction considered*. Return your answer as an array of integers of size `2`, where `answer[0] == arr[i]` and `answer[1] == arr[j]`.

#### Example 1:
<pre>
<strong>Input:</strong> arr = [1,2,3,5], k = 3
<strong>Output:</strong> [2,5]
<strong>Explanation:</strong> The fractions to be considered in sorted order are:
1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
The third fraction is 2/5.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,7], k = 1
<strong>Output:</strong> [1,7]
</pre>

#### Constraints:
* `2 <= arr.length <= 1000`
* <code>1 <= arr[i] <= 3 * 10<sup>4</sup></code>
* `arr[0] == 1`
* `arr[i]` is a **prime** number for `i > 0`.
* All the numbers of `arr` are **unique** and sorted in **strictly increasing** order.
* `1 <= k <= arr.length * (arr.length - 1) / 2`

**Follow up:** Can you solve the problem with better than <code>O(n<sup>2</sup>)</code> complexity?

## Solutions (Rust)

### 1. Solution
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
