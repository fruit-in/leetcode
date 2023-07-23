# @param {Integer[]} arr
# @return {Integer}
def max_chunks_to_sorted(arr)
  x = 0
  ret = 0

  (0...arr.size).each do |i|
    x ^= 1 << arr[i]
    ret += 1 if x == (2 << i) - 1
  end

  ret
end
