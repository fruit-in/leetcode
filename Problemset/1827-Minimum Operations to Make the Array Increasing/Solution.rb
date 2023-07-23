# @param {Integer[]} nums
# @return {Integer}
def min_operations(nums)
  ret = 0

  (1...nums.size).each do |i|
    ret += [nums[i], nums[i - 1] + 1].max - nums[i]
    nums[i] = [nums[i], nums[i - 1] + 1].max
  end

  ret
end
