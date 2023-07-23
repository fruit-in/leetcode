# 304. 二维区域和检索 - 矩阵不可变
给定一个二维矩阵，计算其子矩形范围内元素的总和，该子矩阵的左上角为 (*row*1, *col*1) ，右下角为 (*row*2, *col*2)。

![](https://assets.leetcode-cn.com/aliyun-lc-upload/images/304.png)<br>
上图子矩阵左上角 (row1, col1) = **(2, 1)** ，右下角(row2, col2) = **(4, 3)**，该子矩形内元素的总和为 8。

#### 示例:
```
给定 matrix = [
  [3, 0, 1, 4, 2],
  [5, 6, 3, 2, 1],
  [1, 2, 0, 1, 5],
  [4, 1, 0, 1, 7],
  [1, 0, 3, 0, 5]
]

sumRegion(2, 1, 4, 3) -> 8
sumRegion(1, 1, 2, 2) -> 11
sumRegion(1, 2, 2, 4) -> 12
```

#### 说明:
1. 你可以假设矩阵不可变。
2. 会多次调用 *sumRegion* 方法。
3. 你可以假设 *row*1 ≤ *row*2 且 *col*1 ≤ *col*2。

## 题解 (Rust)

### 1. 题解
```Rust
struct NumMatrix {
    mat: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut mat = matrix.clone();

        for r in 0..matrix.len() {
            let mut row_sum = 0;

            for c in 0..matrix[0].len() {
                if r > 0 {
                    mat[r][c] += mat[r - 1][c];
                }
                mat[r][c] += row_sum;
                row_sum += matrix[r][c];
            }
        }

        Self { mat }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        self.mat[row2][col2]
            + match (row1 > 0, col1 > 0) {
                (false, false) => 0,
                (true, false) => -self.mat[row1 - 1][col2],
                (false, true) => -self.mat[row2][col1 - 1],
                _ => {
                    self.mat[row1 - 1][col1 - 1]
                        - self.mat[row1 - 1][col2]
                        - self.mat[row2][col1 - 1]
                }
            }
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
```
