# @param {String} s
# @return {Integer}
def balanced_string(s)
  count = {}
  count.default = 0
  l = 0
  ret = s.size

  s.each_char { |c| count[c] += 1 }

  (0..s.size).each do |r|
    while l <= r && 4 * count.values.max - count.values.sum <= r - l
      ret = [ret, r - l].min
      count[s[[l, s.size - 1].min]] += 1
      l += 1
    end
    count[s[[r, s.size - 1].min]] -= 1
  end

  ret
end
