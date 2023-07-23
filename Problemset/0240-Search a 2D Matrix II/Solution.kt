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
