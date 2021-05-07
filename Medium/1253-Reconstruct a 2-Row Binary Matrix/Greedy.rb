# @param {Integer} upper
# @param {Integer} lower
# @param {Integer[]} colsum
# @return {Integer[][]}
def reconstruct_matrix(upper, lower, colsum)
  ret = Array.new(2) { [0] * colsum.size }

  (0...colsum.size).each do |i|
    if colsum[i] == 2
      ret[0][i] = 1
      ret[1][i] = 1
      upper -= 1
      lower -= 1
    elsif colsum[i] == 1
      if upper > lower
        ret[0][i] = 1
        upper -= 1
      else
        ret[1][i] = 1
        lower -= 1
      end
    end
  end

  upper | lower == 0 ? ret : []
end
