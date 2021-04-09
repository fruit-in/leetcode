# @param {Integer[]} gain
# @return {Integer}
def largest_altitude(gain)
  sum = 0
  ret = 0

  gain.each do |x|
    sum += x
    ret = sum if sum > ret
  end

  ret
end
