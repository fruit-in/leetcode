# @param {Integer} n
# @return {Integer}
def num_trees(n)
    dp = [0] * (n + 1)
    dp[0] = 1

    for i in 1..n
        for j in 1..i
            dp[i] += dp[j - 1] * dp[i - j]
        end
    end

    return dp[n]
end
