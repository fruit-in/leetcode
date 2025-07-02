# 835. 图像重叠
给你两个图像 `img1` 和 `img2` ，两个图像的大小都是 `n x n` ，用大小相同的二进制正方形矩阵表示。二进制矩阵仅由若干 `0` 和若干 `1` 组成。

**转换** 其中一个图像，将所有的 `1` 向左，右，上，或下滑动任何数量的单位；然后把它放在另一个图像的上面。该转换的 **重叠** 是指两个图像 **都** 具有 `1` 的位置的数目。

请注意，转换 **不包括** 向任何方向旋转。越过矩阵边界的 `1` 都将被清除。

最大可能的重叠数量是多少？

#### 示例 1:
![](https://assets.leetcode.com/uploads/2020/09/09/overlap1.jpg)
<pre>
<strong>输入:</strong> img1 = [[1,1,0],[0,1,0],[0,1,0]], img2 = [[0,0,0],[0,1,1],[0,0,1]]
<strong>输出:</strong> 3
<strong>解释:</strong> 将 img1 向右移动 1 个单位，再向下移动 1 个单位。
<img src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step1.jpg">
两个图像都具有 1 的位置的数目是 3（用红色标识）。
<img src="https://assets.leetcode.com/uploads/2020/09/09/overlap_step2.jpg">
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> img1 = [[1]], img2 = [[1]]
<strong>输出:</strong> 1
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> img1 = [[0]], img2 = [[0]]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `n == img1.length == img1[i].length`
* `n == img2.length == img2[i].length`
* `1 <= n <= 30`
* `img1[i][j]` 为 `0` 或 `1`
* `img2[i][j]` 为 `0` 或 `1`

## 题解 (Rust)

### 1. 题解
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
