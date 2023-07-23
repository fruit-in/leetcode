# @param {String} s
# @param {String} t
# @param {Integer} max_cost
# @return {Integer}
def equal_substring(s, t, max_cost)
  s = s.bytes
  t = t.bytes
  i = -1
  cost = 0
  ret = 0

  (0...s.size).each do |j|
    cost += (s[j] - t[j]).abs
    while cost > max_cost
      i += 1
      cost -= (s[i] - t[i]).abs
    end
    ret = [ret, j - i].max
  end

  ret
end
