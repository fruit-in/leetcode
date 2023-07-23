# 54. 螺旋矩阵
给定一个包含 *m* x *n* 个元素的矩阵（*m* 行, *n* 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。

#### 示例 1:
<pre>
<strong>输入:</strong>
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
<strong>输出:</strong> [1,2,3,6,9,8,7,4,5]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
[
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9,10,11,12]
]
<strong>输出:</strong> [1,2,3,4,8,12,11,10,9,5,6,7]
</pre>

## 题解 (Rust)

### 1. 模拟
```Rust
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return Vec::new();
        }

        let mut row = 0;
        let mut col = 0;
        let mut u = 0;
        let mut d = matrix.len() as i32 - 1;
        let mut l = 0;
        let mut r = matrix[0].len() as i32 - 1;
        let mut dir = (0, 1);
        let mut ret = Vec::new();

        for _ in 0..(matrix.len() * matrix[0].len()) {
            ret.push(matrix[row as usize][col as usize]);

            match dir {
                (0, 1) if col == r => { dir = (1, 0); u += 1; },
                (1, 0) if row == d => { dir = (0, -1); r -= 1; },
                (0, -1) if col == l => { dir = (-1, 0); d -= 1; },
                (-1, 0) if row == u => { dir = (0, 1); l += 1; },
                _ => (),
            }

            row += dir.0;
            col += dir.1;
        }

        ret
    }
}
```
