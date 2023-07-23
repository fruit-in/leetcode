# @param {Integer[]} nums
# @param {Integer} target
# @return {Integer}
def find_target_sum_ways(nums, target)
  sum = nums.sum
  return 0 if sum < target.abs

  dp0 = [0] * (2 * sum + 1)
  dp0[sum] = 1

  nums.each do |num|
    dp1 = [0] * (2 * sum + 1)

    (0..2 * sum).each do |i|
      if dp0[i] > 0
        dp1[i + num] += dp0[i]
        dp1[i - num] += dp0[i]
      end
    end

    dp0 = dp1
  end

  dp0[sum + target]
end
