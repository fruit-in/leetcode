# @param {Integer[]} arr
# @return {Integer}
def num_of_subarrays(arr)
  sum = 0
  odd = 0
  even = 1
  ret = 0

  arr.each do |x|
    sum += x
    if sum.odd?
      odd += 1
      ret += even
    else
      even += 1
      ret += odd
    end
  end

  ret % 1_000_000_007
end
