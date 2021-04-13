# @param {Integer[]} nums
# @return {Boolean}
def increasing_triplet(nums)
  min3 = [nil, nil, nil]

  nums.each do |x|
    if min3[1].nil?
      min3[1] = x
    elsif min3[2].nil? && x <= min3[1]
      min3[1] = x
    elsif min3[2].nil?
      min3[2] = x
    elsif x > min3[2]
      return true
    elsif x > min3[1]
      min3[2] = x
    elsif min3[0].nil? && x < min3[1]
      min3[0] = x
    elsif !min3[0].nil? && x > min3[0]
      min3 = [nil, min3[0], x]
    elsif !min3[0].nil? && x < min3[0]
      min3[0] = x
    end
  end

  false
end
