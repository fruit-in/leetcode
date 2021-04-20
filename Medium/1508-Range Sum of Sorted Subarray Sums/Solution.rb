# @param {Integer[]} nums
# @param {Integer} n
# @param {Integer} left
# @param {Integer} right
# @return {Integer}
def range_sum(nums, _n, left, right)
  sums = []
  ret = 0

  (0...nums.size).each do |i|
    sum = 0
    (i...nums.size).each do |j|
      sum += nums[j]
      sums.push(sum)
    end
  end

  sums.sort!

  (left..right).each do |i|
    ret = (ret + sums[i - 1]) % 1_000_000_007
  end

  ret
end
