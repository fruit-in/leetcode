# @param {Integer[][]} grid
# @return {Integer}
def count_servers(grid)
  row_count = [0] * grid.length
  col_count = [0] * grid[0].length
  ret = 0

  (0...grid.length).each do |r|
    (0...grid[0].length).each do |c|
      if grid[r][c] == 1
        row_count[r] += 1
        col_count[c] += 1
      end
    end
  end

  (0...grid.length).each do |r|
    (0...grid[0].length).each do |c|
      ret += 1 if grid[r][c] == 1 && (row_count[r] > 1 || col_count[c] > 1)
    end
  end

  ret
end
