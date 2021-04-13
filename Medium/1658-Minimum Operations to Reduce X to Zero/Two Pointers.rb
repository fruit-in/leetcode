# @param {Integer[]} nums
# @param {Integer} x
# @return {Integer}
def min_operations(nums, x)
  l = nums.size
  r = 0
  sum = nums.sum
  ret = -1

  while r < nums.size
    ret = l + r if sum == x && (ret == -1 || l + r < ret)
    if (sum > x && l > 0) || l + r >= nums.size
      l -= 1
      sum -= nums[l]
    else
      r += 1
      sum += nums[-r]
    end
  end

  ret
end
