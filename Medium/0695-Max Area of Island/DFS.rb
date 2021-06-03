# @param {Integer[][]} grid
# @return {Integer}
def max_area_of_island(grid)
  m = grid.size
  n = grid[0].size
  ret = 0

  (0...m).each do |i|
    (0...n).each do |j|
      next if grid[i][j] == 0

      area = 0
      cells = [[i, j]]

      until cells.empty?
        i, j = cells.pop
        next if grid[i][j] == 0

        area += 1
        grid[i][j] = 0

        cells.push([i - 1, j]) if i > 0 && grid[i - 1][j] == 1
        cells.push([i + 1, j]) if i < m - 1 && grid[i + 1][j] == 1
        cells.push([i, j - 1]) if j > 0 && grid[i][j - 1] == 1
        cells.push([i, j + 1]) if j < n - 1 && grid[i][j + 1] == 1
      end

      ret = [ret, area].max
    end
  end

  ret
end
