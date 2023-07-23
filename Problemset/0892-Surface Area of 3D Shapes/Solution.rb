# @param {Integer[][]} grid
# @return {Integer}
def surface_area(grid)
    area = 0

    for i in 0...grid.length
        for j in 0...grid[0].length
            if grid[i][j] > 0
                area += grid[i][j] * 2 + 1
                area -= [grid[i - 1][j], grid[i][j]].min if i > 0
                area -= [grid[i][j - 1], grid[i][j]].min if j > 0
            end
        end
    end

    return area * 2
end
