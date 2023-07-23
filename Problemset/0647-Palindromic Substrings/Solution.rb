# @param {String} s
# @return {Integer}
def count_substrings(s)
  ret = 0

  (0...s.size).each do |i|
    (0..[i, s.size - 1 - i].min).each do |j|
      if s[i - j] == s[i + j]
        ret += 1
      else
        break
      end
    end
    next unless i < s.size - 1 && s[i] == s[i + 1]

    (0..[i, s.size - 2 - i].min).each do |j|
      if s[i - j] == s[i + 1 + j]
        ret += 1
      else
        break
      end
    end
  end

  ret
end
