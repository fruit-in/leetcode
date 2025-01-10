# 1439. Find the Kth Smallest Sum of a Matrix With Sorted Rows
You are given an `m x n` matrix `mat` that has its rows sorted in non-decreasing order and an integer `k`.

You are allowed to choose **exactly one element** from each row to form an array.

Return *the* <code>k<sup>th</sup></code> *smallest array sum among all possible arrays*.

#### Example 1:
<pre>
<strong>Input:</strong> mat = [[1,3,11],[2,4,6]], k = 5
<strong>Output:</strong> 7
<strong>Explanation:</strong> Choosing one element from each row, the first k smallest sum are:
[1,2], [1,4], [3,2], [3,4], [1,6]. Where the 5th sum is 7.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> mat = [[1,3,11],[2,4,6]], k = 9
<strong>Output:</strong> 17
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> mat = [[1,10,10],[1,4,5],[2,3,6]], k = 7
<strong>Output:</strong> 9
<strong>Explanation:</strong> Choosing one element from each row, the first k smallest sum are:
[1,1,2], [1,1,3], [1,4,2], [1,4,3], [1,1,6], [1,5,2], [1,5,3]. Where the 7th sum is 9.
</pre>

#### Constraints:
* `m == mat.length`
* `n == mat.length[i]`
* `1 <= m, n <= 40`
* `1 <= mat[i][j] <= 5000`
* <code>1 <= k <= min(200, n<sup>m</sup>)</code>
* `mat[i]` is a non-decreasing array.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(mat: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::from([0]);

        for i in 0..mat.len() {
            let mut tmp = BinaryHeap::new();

            while let Some(x) = heap.pop() {
                for j in 0..mat[0].len() {
                    tmp.push(x + mat[i][j]);

                    if tmp.len() as i32 > k {
                        tmp.pop();
                    }
                }
            }

            heap = tmp;
        }

        heap.pop().unwrap()
    }
}
```
