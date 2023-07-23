# @param {Integer[][]} mat
# @return {Integer[][]}
def diagonal_sort(mat)
    m, n = mat.length, mat[0].length

    for i in 0...(m + n)
        row = [m - 1 - i, 0].max
        col = [i - m + 1, 0].max
        arr = []

        for j in 0...[m - row, n - col].min
            arr.push(mat[row + j][col + j])
        end

        arr.sort!

        for j in 0...[m - row, n - col].min
            mat[row + j][col + j] = arr[j]
        end
    end

    return mat
end
