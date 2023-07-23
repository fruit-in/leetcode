# @param {Integer[]} nums
# @param {Integer[]} l
# @param {Integer[]} r
# @return {Boolean[]}
def check_arithmetic_subarrays(nums, l, r)
  ret = [false] * l.size

  (0...l.size).each do |i|
    sub = nums[l[i]..r[i]].sort
    ret[i] = sub.size > 1 && (2...sub.size).all? { |j| sub[j] - sub[j - 1] == sub[1] - sub[0] }
  end

  ret
end
