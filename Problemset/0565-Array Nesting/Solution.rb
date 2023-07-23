# @param {Integer[]} nums
# @return {Integer}
def array_nesting(nums)
  ret = 1

  (0...nums.size).each do |i|
    next if nums[i] < 0

    length = 0
    j = i
    while nums[j] >= 0
      nums[j] = -nums[j] - 1
      length += 1
      j = -nums[j] - 1
    end

    ret = [ret, length].max
  end

  ret
end
