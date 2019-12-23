# 766. 托普利茨矩阵
如果一个矩阵的每一方向由左上到右下的对角线上具有相同元素，那么这个矩阵是*托普利茨矩阵*。

给定一个 ```M x N``` 的矩阵，当且仅当它是*托普利茨矩阵*时返回 ```True```。

#### 示例 1:
<pre>
<strong>输入:</strong>
matrix = [
  [1,2,3,4],
  [5,1,2,3],
  [9,5,1,2]
]
<strong>输出:</strong> True
<strong>解释:</strong>
在上述矩阵中, 其对角线为:
"[9]", "[5, 5]", "[1, 1, 1]", "[2, 2, 2]", "[3, 3]", "[4]"。
各条对角线上的所有元素均相同, 因此答案是True。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong>
matrix = [
  [1,2],
  [2,2]
]
<strong>输出:</strong> False
<strong>解释:</strong>
对角线"[1, 2]"上的元素不同。
</pre>

#### 说明:
1. ```matrix``` 是一个包含整数的二维数组。
2. ```matrix``` 的行数和列数均在 ```[1, 20]```范围内。
3. ```matrix[i][j]``` 包含的整数在 ```[0, 99]```范围内。

#### 进阶:
1. 如果矩阵存储在磁盘上，并且磁盘内存是有限的，因此一次最多只能将一行矩阵加载到内存中，该怎么办？
2. 如果矩阵太大以至于只能一次将部分行加载到内存中，该怎么办？

## 题解 (Rust)

### 1. 题解
```Rust
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        for i in 1..matrix.len() {
            for j in 1..matrix[0].len() {
                if matrix[i - 1][j - 1] != matrix[i][j] {
                    return false;
                }
            }
        }

        true
    }
}
```
