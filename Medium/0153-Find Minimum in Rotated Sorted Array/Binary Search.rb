# @param {Integer[]} nums
# @return {Integer}
def find_min(nums)
  l = 0
  r = nums.size - 1

  while l <= r
    m = (l + r) / 2

    if m > 0 && nums[m - 1] > nums[m]
      return nums[m]
    elsif nums[m] >= nums[0]
      l = m + 1
    else
      r = m - 1
    end
  end

  nums[0]
end
