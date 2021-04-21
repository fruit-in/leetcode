# @param {Integer} n
# @return {Integer}
def knight_dialer(n)
  dp = [1] * 10

  (1...n).each do |_|
    dp = [
      dp[4] + dp[6],
      dp[6] + dp[8],
      dp[7] + dp[9],
      dp[4] + dp[8],
      dp[0] + dp[3] + dp[9],
      0,
      dp[0] + dp[1] + dp[7],
      dp[2] + dp[6],
      dp[1] + dp[3],
      dp[2] + dp[4]
    ]
  end

  dp.sum % 1_000_000_007
end
