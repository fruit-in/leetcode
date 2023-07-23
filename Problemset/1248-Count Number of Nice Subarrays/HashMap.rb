# @param {Integer[]} nums
# @param {Integer} k
# @return {Integer}
def number_of_subarrays(nums, k)
  counter = { 0 => 1 }
  counter.default = 0
  count = 0
  ret = 0

  nums.each do |x|
    count += 1 if x.odd?
    ret += counter[count - k]
    counter[count] += 1
  end

  ret
end
