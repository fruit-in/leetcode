# @param {Integer[]} nums
# @return {Integer}
def array_sign(nums)
  sign = 1

  nums.each do |x|
    return 0 if x == 0

    sign *= -1 if x < 0
  end

  sign
end
