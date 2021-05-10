# @param {Integer[]} nums
# @return {Boolean}
def is_ideal_permutation(nums)
  max = 0

  (2...nums.size).each do |i|
    max = [max, nums[i - 2]].max
    return false if nums[i] < max
  end

  true
end
