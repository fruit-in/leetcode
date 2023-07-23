# @param {Integer} n
# @return {Integer}
def count_vowel_strings(n)
  dp = [1] * 5

  (1...n).each do |_|
    dp[1] += dp[0]
    dp[2] += dp[1]
    dp[3] += dp[2]
    dp[4] += dp[3]
  end

  dp.sum
end
