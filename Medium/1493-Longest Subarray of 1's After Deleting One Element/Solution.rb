# @param {Integer[]} nums
# @return {Integer}
def longest_subarray(nums)
  zeros = 0
  l = 0
  ret = 0

  (0...nums.size).each do |r|
    zeros += 1 if nums[r] == 0
    while zeros > 1
      zeros -= 1 if nums[l] == 0
      l += 1
    end
    ret = [ret, r - l - zeros + 1].max
  end

  [ret, nums.size - 1].min
end
