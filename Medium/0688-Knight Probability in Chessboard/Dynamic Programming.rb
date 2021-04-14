# @param {Integer} n
# @param {Integer} k
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def knight_probability(n, k, r, c)
  dp = Array.new(n) { [0] * n }
  dp[r][c] = 1

  (1..k).each do |_|
    dp_ = Array.new(n) { [0] * n }
    (0...n).each do |r|
      (0...n).each do |c|
        [[-2, 1], [-1, 2], [1, 2], [2, 1], [2, -1], [1, -2], [-1, -2], [-2, -1]].each do |dr, dc|
          dp_[r][c] += probability(dp, n, r + dr, c + dc) / 8.0
        end
      end
    end
    dp = dp_
  end

  dp.flatten.sum
end

# @param {Integer[][]} dp
# @param {Integer} n
# @param {Integer} r
# @param {Integer} c
# @return {Float}
def probability(dp, n, r, c)
  r < 0 || c < 0 || r >= n || c >= n ? 0.0 : dp[r][c]
end
