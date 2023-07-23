# @param {Integer[]} nums
# @param {Integer} limit
# @param {Integer} goal
# @return {Integer}
def min_elements(nums, limit, goal)
  ((nums.sum - goal).abs * 1.0 / limit).ceil
end
