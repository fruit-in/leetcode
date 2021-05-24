# @param {Integer[]} nums
# @return {Integer}
def max_ascending_sum(nums)
  sum = nums[0]
  ret = sum

  (1...nums.size).each do |i|
    sum = nums[i] + (nums[i] > nums[i - 1] ? sum : 0)
    ret = [ret, sum].max
  end

  ret
end
