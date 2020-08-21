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
