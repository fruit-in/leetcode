# 74. 搜索二维矩阵
编写一个高效的算法来判断 *m* x *n* 矩阵中，是否存在一个目标值。该矩阵具有如下特性：
* 每行中的整数从左到右按升序排列。
* 每行的第一个整数大于前一行的最后一个整数。

#### 示例 1:
<pre>
<strong>输入:</strong>
matrix = [
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
]
target = 3
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
matrix = [
  [1,   3,  5,  7],
  [10, 11, 16, 20],
  [23, 30, 34, 50]
]
target = 13
<strong>输出:</strong> false
</pre>

## 题解 (Rust)

### 1. 二分查找
```Rust
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut l = 0;
        let mut r = m * n;

        while l < r {
            let mid = (l + r) / 2;
            let row = mid / n;
            let col = mid % n;

            if target == matrix[row][col] {
                return true;
            } else if target < matrix[row][col] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }

        false
    }
}
```
