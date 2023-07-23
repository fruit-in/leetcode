# @param {String} s
# @return {String}
def longest_nice_substring(s)
  ret = ''

  (0...s.size).each do |i|
    counter = [0] * 26

    (i...s.size).each do |j|
      if ('A'..'Z').include?(s[j])
        counter[s[j].ord - 65] |= 1
      else
        counter[s[j].ord - 97] |= 2
      end
      ret = s[i..j] if counter.all? { |c| c % 3 == 0 } && j - i + 1 > ret.size
    end
  end

  ret
end
