# @param {Integer[]} nums
# @return {Integer}
def min_difference(nums)
  return 0 if nums.length < 5

  maxs = nums.max(4)
  mins = nums.min(4)

  [maxs[0] - mins[3], maxs[1] - mins[2], maxs[2] - mins[1], maxs[3] - mins[0]].min
end
