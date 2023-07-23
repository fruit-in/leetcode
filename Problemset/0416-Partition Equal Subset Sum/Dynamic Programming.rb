# @param {Integer[]} nums
# @return {Boolean}
def can_partition(nums)
  sum = nums.sum
  target = sum / 2
  return false if sum.odd? || nums.any? { |x| x > target }

  dp = [false] * (target + 1)
  dp[0] = true

  nums.each { |x| (0..target - x).reverse_each { |i| dp[i + x] |= dp[i] } }

  dp[target]
end
