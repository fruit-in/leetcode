# @param {Integer[]} arr
# @param {Integer} k
# @return {Integer}
def k_concatenation_max_sum(arr, k)
  l_sum = 0
  r_sum = 0
  l_max_sum = 0
  r_max_sum = 0
  l_min_sum = 0
  ret = 0

  (1..arr.size).each do |i|
    l_sum += arr[i - 1]
    r_sum += arr[-i]
    l_max_sum = [l_max_sum, l_sum].max
    r_max_sum = [r_max_sum, r_sum].max
    l_min_sum = [l_min_sum, l_sum].min
    ret = [ret, l_sum - l_min_sum].max
  end

  k == 1 ? ret : [ret, [l_sum, 0].max * (k - 2) + l_max_sum + r_max_sum].max % 1_000_000_007
end
