# @param {Integer[]} nums
# @return {Integer}
def ways_to_make_fair(nums)
  even_sums = [nums[0]]
  odd_sums = [0]
  ret = 0

  (1...nums.length).each do |i|
    if i.even?
      even_sums.push(nums[i] + even_sums[i - 1])
      odd_sums.push(odd_sums[i - 1])
    else
      even_sums.push(even_sums[i - 1])
      odd_sums.push(nums[i] + odd_sums[i - 1])
    end
  end

  (0...nums.length).each do |i|
    even_sum = even_sums[i] + odd_sums[-1] - odd_sums[i] - (i.even? ? nums[i] : 0)
    odd_sum = odd_sums[i] + even_sums[-1] - even_sums[i] - (i.even? ? 0 : nums[i])

    ret += 1 if even_sum == odd_sum
  end

  ret
end
