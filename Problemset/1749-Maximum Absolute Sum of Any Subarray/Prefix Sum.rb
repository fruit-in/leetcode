# @param {Integer[]} nums
# @return {Integer}
def max_absolute_sum(nums)
  sum = 0
  max_sum = 0
  min_sum = 0
  ret = 0

  nums.each do |x|
    sum += x
    max_sum = [max_sum, sum].max
    min_sum = [min_sum, sum].min
    ret = [ret, (sum - max_sum).abs, (sum - min_sum).abs].max
  end

  ret
end
