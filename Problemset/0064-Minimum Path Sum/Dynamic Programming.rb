# @param {Integer[][]} grid
# @return {Integer}
def min_path_sum(grid)
    m, n = grid.length, grid[0].length

    for i in 0...m
        for j in 0...n
            if i == 0 and j != 0
                grid[i][j] += grid[i][j - 1]
            elsif i != 0 and j == 0
                grid[i][j] += grid[i - 1][j]
            elsif i != 0 and j != 0
                grid[i][j] += [grid[i][j - 1], grid[i - 1][j]].min
            end
        end
    end

    return grid[m - 1][n - 1]
end
