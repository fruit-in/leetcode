# 240. 搜索二维矩阵 II
编写一个高效的算法来搜索 *m* x *n* 矩阵 matrix 中的一个目标值 target。该矩阵具有以下特性：
* 每行的元素从左到右升序排列。
* 每列的元素从上到下升序排列。

#### 示例:
现有矩阵 matrix 如下：
```
[
  [1,   4,  7, 11, 15],
  [2,   5,  8, 12, 19],
  [3,   6,  9, 16, 22],
  [10, 13, 14, 17, 24],
  [18, 21, 23, 26, 30]
]
```

给定 target = `5`，返回 `true`。

给定 target = `20`，返回 `false`。

## 题解 (Ruby)

### 1. 题解
```Ruby
# @param {Integer[][]} matrix
# @param {Integer} target
# @return {Boolean}
def search_matrix(matrix, target)
    return false if matrix.empty? or matrix[0].empty?

    row, col = matrix.length - 1, 0

    while row >= 0 and col < matrix[0].length
        if matrix[row][col] < target
            col += 1
        elsif matrix[row][col] > target
            row -= 1
        else
            return true
        end
    end

    return false
end
```

## 题解 (Kotlin)

### 1. 题解
```Kotlin
class Solution {
    fun searchMatrix(matrix: Array<IntArray>, target: Int): Boolean {
        if (matrix.isEmpty() || matrix[0].isEmpty()) {
            return false
        }

        var row = matrix.size - 1
        var col = 0

        while (row >= 0 && col < matrix[0].size) {
            if (matrix[row][col] < target) {
                col += 1
            } else if (matrix[row][col] > target) {
                row -= 1
            } else {
                return true
            }
        }

        return false
    }
}
```
