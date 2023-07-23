# 39. 组合总和
给定一个**无重复元素**的数组 `candidates` 和一个目标数 `target` ，找出 `candidates` 中所有可以使数字和为 `target` 的组合。

`candidates` 中的数字可以无限制重复被选取。

#### 说明:
* 所有数字（包括 `target`）都是正整数。
* 解集不能包含重复的组合。

#### 示例 1:
<pre>
<strong>输入:</strong> candidates = [2,3,6,7], target = 7,
<strong>所求解集为:</strong>
[
  [7],
  [2,2,3]
]
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> candidates = [2,3,5], target = 8,
<strong>所求解集为:</strong>
[
  [2,2,2,2],
  [2,3,3],
  [3,5]
]
</pre>

#### 提示:
* `1 <= candidates.length <= 30`
* `1 <= candidates[i] <= 200`
* `candidate` 中的每个元素都是独一无二的。
* `1 <= target <= 500`

## 题解 (Ruby)

### 1. 动态规划
```Ruby
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
```
