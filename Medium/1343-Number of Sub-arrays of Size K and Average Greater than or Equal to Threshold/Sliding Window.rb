# @param {Integer[]} arr
# @param {Integer} k
# @param {Integer} threshold
# @return {Integer}
def num_of_subarrays(arr, k, threshold)
  sum = arr[...k].sum
  i = 0
  ret = sum >= k * threshold ? 1 : 0

  (k...arr.size).each do |j|
    sum += arr[j] - arr[i]
    i += 1
    ret += 1 if sum >= k * threshold
  end

  ret
end
