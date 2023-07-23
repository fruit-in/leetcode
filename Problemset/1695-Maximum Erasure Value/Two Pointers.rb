# @param {Integer[]} nums
# @return {Integer}
def maximum_unique_subarray(nums)
  i = 0
  counter = {}
  counter.default = 0
  sum = 0
  ret = 0

  (0...nums.size).each do |j|
    counter[nums[j]] += 1
    sum += nums[j]
    while counter[nums[j]] > 1
      counter[nums[i]] -= 1
      sum -= nums[i]
      i += 1
    end
    ret = [ret, sum].max
  end

  ret
end
