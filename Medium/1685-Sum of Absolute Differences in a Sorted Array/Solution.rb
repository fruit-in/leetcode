# @param {Integer[]} nums
# @return {Integer[]}
def get_sum_absolute_differences(nums)
  total_sum = nums.sum
  left_sum = 0
  result = [0] * nums.size

  (0...nums.size).each do |i|
    result[i] = (2 * i - nums.size) * nums[i] + total_sum - 2 * left_sum
    left_sum += nums[i]
  end

  result
end
