# 835. Image Overlap
You are given two images, `img1` and `img2`, represented as binary, square matrices of size `n x n`. A binary matrix has only `0`s and `1`s as values.

We **translate** one image however we choose by sliding all the `1` bits left, right, up, and/or down any number of units. We then place it on top of the other image. We can then calculate the **overlap** by counting the number of positions that have a `1` in **both** images.

Note also that a translation does **not** include any kind of rotation. Any `1` bits that are translated outside of the matrix borders are erased.

Return *the largest possible overlap*.

#### Example 1:
![](https://assets.leetcode.com/uploads/2020/09/09/overlap1.jpg)
<pre>
<strong>Input:</strong> img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
<strong>Output:</strong> 3
<strong>Explanation:</strong> We translate img1 to right by 1 unit and down by 1 unit.
<img src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step1.jpg">
The number of positions that have a 1 in both images is 3 (shown in red).
<img src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step2.jpg">
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> img1 = [[1]], img2 = [[1]]
<strong>Output:</strong> 1
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> img1 = [[0]], img2 = [[0]]
<strong>Output:</strong> 0
</pre>

#### Constraints:
* `n == img1.length == img1[i].length`
* `n == img2.length == img2[i].length`
* `1 <= n <= 30`
* `img1[i][j]` is either `0` or `1`.
* `img2[i][j]` is either `0` or `1`.

## Solutions (Rust)

### 1. Solution
```Rust
use std::collections::VecDeque;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let n = img1.len();
        let mut mask1 = VecDeque::from(vec![0; n]);
        let mut mask2 = VecDeque::from(vec![0; n]);
        let mut ret = 0;

        for i in 0..n {
            for j in 0..n {
                mask1[i] |= img1[i][j] << j;
                mask2[i] |= img2[i][j] << j;
            }
        }

        let mut tmp = mask1.clone();

        for _ in 0..n {
            for i in 0..n {
                let mut left = 0;
                let mut right = 0;

                for j in 0..n {
                    left += ((tmp[j] << i) & mask2[j]).count_ones();
                    right += ((tmp[j] >> i) & mask2[j]).count_ones();
                }

                ret = ret.max(left).max(right);
            }

            tmp[0] = 0;
            tmp.rotate_left(1);
        }

        tmp = mask1.clone();

        for _ in 0..n {
            for i in 0..n {
                let mut left = 0;
                let mut right = 0;

                for j in 0..n {
                    left += ((tmp[j] << i) & mask2[j]).count_ones();
                    right += ((tmp[j] >> i) & mask2[j]).count_ones();
                }

                ret = ret.max(left).max(right);
            }

            tmp.rotate_right(1);
            tmp[0] = 0;
        }

        ret as i32
    }
}
```
