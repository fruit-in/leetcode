# @param {String} s
# @return {Integer}
def min_deletions(s)
  counter = [0] * 26
  ret = 0

  s.each_byte { |c| counter[c - 97] += 1 }
  counter.sort!

  (25..1).step(-1).each do |i|
    if counter[i] <= counter[i - 1]
      ret += [counter[i - 1] - counter[i] + 1, counter[i - 1]].min
      counter[i - 1] = [counter[i] - 1, 0].max
    end
  end

  ret
end
