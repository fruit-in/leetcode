# @param {Integer[]} candidates
# @param {Integer} target
# @return {Integer[][]}
def combination_sum(candidates, target)
  dp = Array.new(target + 1) { [] }
  candidates.sort!.reverse!

  candidates.each do |x|
    next unless x <= target

    dp[x].push([x])
    (x..target - x).each do |i|
      dp[i].each do |c|
        dp[i + x].push(c + [x])
      end
    end
  end

  dp[target]
end
