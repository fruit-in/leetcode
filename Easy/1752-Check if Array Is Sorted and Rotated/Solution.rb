# @param {Integer[]} nums
# @return {Boolean}
def check(nums)
  count = (1...nums.size).count { |i| nums[i] < nums[i - 1] }
  count += 1 if nums[0] < nums[-1]

  count < 2
end
