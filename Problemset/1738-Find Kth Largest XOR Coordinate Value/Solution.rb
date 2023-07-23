# @param {Integer[][]} matrix
# @param {Integer} k
# @return {Integer}
def kth_largest_value(matrix, k)
  vals = []

  (0...matrix.size).each do |i|
    (0...matrix[0].size).each do |j|
      matrix[i][j] ^= matrix[i - 1][j] if i > 0
      matrix[i][j] ^= matrix[i][j - 1] if j > 0
      matrix[i][j] ^= matrix[i - 1][j - 1] if i > 0 && j > 0
      vals.push(matrix[i][j])
    end
  end

  vals.sort[-k]
end
