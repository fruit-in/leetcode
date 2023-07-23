# @param {String} s
# @param {String[]} word_dict
# @return {Boolean}
def word_break(s, word_dict)
  dp = [true] + [false] * s.size

  (1..s.size).each do |i|
    (0...i).each do |j|
      if dp[j] && word_dict.include?(s[j...i])
        dp[i] = true
        break
      end
    end
  end

  dp[s.size]
end
