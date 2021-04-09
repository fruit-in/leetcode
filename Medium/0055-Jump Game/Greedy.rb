# @param {Integer[]} nums
# @return {Boolean}
def can_jump(nums)
  max_index = 0

  (0...nums.size).each do |i|
    return false if i > max_index

    max_index = [max_index, i + nums[i]].max
  end

  true
end
