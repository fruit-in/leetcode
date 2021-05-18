# @param {Integer[]} nums
# @return {Integer}
def find_max_length(nums)
  count = 0
  counts = { 0 => -1 }
  ret = 0

  (0...nums.size).each do |i|
    count += nums[i] == 0 ? 1 : -1
    counts[count] = i unless counts.include?(count)
    ret = [ret, i - counts[count]].max
  end

  ret
end
