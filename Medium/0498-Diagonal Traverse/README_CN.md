# 498. 对角线遍历
给定一个含有 M x N 个元素的矩阵（M 行，N 列），请以对角线遍历的顺序返回这个矩阵中的所有元素，对角线遍历如下图所示。

#### 示例:
<pre>
<strong>输入:</strong>
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
<strong>输出:</strong> [1,2,4,7,5,3,6,8,9]
<strong>解释:</strong>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/diagonal_traverse.png">
</pre>

#### 说明:
1. 给定矩阵中的元素总数不会超过 100000 。

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return Vec::new();
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut up_right = true;
        let mut ret = Vec::new();

        for i in 0..(m + n - 1) {
            if up_right {
                let x = i.min(m - 1);
                let y = i - x;
                for j in 0..=(x.min(n - 1 - y)) {
                    ret.push(matrix[x - j][y + j]);
                }
            } else {
                let y = i.min(n - 1);
                let x = i - y;
                for j in 0..=(y.min(m - 1 - x)) {
                    ret.push(matrix[x + j][y - j]);
                }
            }

            up_right = !up_right;
        }

        ret
    }
}
```
