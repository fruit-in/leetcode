# 240. Search a 2D Matrix II
Write an efficient algorithm that searches for a value in an *m* x *n* matrix. This matrix has the following properties:
* Integers in each row are sorted in ascending from left to right.
* Integers in each column are sorted in ascending from top to bottom.

#### Example:
Consider the following matrix:
```
[
  [1,   4,  7, 11, 15],
  [2,   5,  8, 12, 19],
  [3,   6,  9, 16, 22],
  [10, 13, 14, 17, 24],
  [18, 21, 23, 26, 30]
]
```

Given target = `5`, return `true`.

Given target = `20`, return `false`.

## Solutions (Ruby)

### 1. Solution
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

## Solutions (Kotlin)

### 1. Solution
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
