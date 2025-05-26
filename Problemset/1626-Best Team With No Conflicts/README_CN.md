# 1626. 无矛盾的最佳球队
假设你是球队的经理。对于即将到来的锦标赛，你想组合一支总体得分最高的球队。球队的得分是球队中所有球员的分数 **总和** 。

然而，球队中的矛盾会限制球员的发挥，所以必须选出一支 **没有矛盾** 的球队。如果一名年龄较小球员的分数 **严格大于** 一名年龄较大的球员，则存在矛盾。同龄球员之间不会发生矛盾。

给你两个列表 `scores` 和 `ages`，其中每组 `scores[i]` 和 `ages[i]` 表示第 `i` 名球员的分数和年龄。请你返回 **所有可能的无矛盾球队中得分最高那支的分数** 。

#### 示例 1:
<pre>
<strong>输入:</strong> scores = [1,3,5,10,15], ages = [1,2,3,4,5]
<strong>输出:</strong> 34
<strong>解释:</strong> 你可以选中所有球员。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> scores = [4,5,6,5], ages = [2,1,2,1]
<strong>输出:</strong> 16
<strong>解释:</strong> 最佳的选择是后 3 名球员。注意，你可以选中多个同龄球员。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> scores = [1,2,3,5], ages = [8,9,10,1]
<strong>输出:</strong> 6
<strong>解释:</strong> 最佳的选择是前 3 名球员。
</pre>

#### 提示:
* `1 <= scores.length, ages.length <= 1000`
* `scores.length == ages.length`
* <code>1 <= scores[i] <= 10<sup>6</sup></code>
* `1 <= ages[i] <= 1000`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def bestTeamScore(self, scores: List[int], ages: List[int]) -> int:
        players = sorted(zip(ages, scores))
        sl = SortedList([(0, 0)])

        for age, score in players:
            i = sl.bisect_right((score, inf))
            sl.add((score, sl[i - 1][1] + score))
            while i + 1 < len(sl) and sl[i + 1][1] <= sl[i][1]:
                sl.pop(i + 1)
            if sl[i - 1][0] == score:
                sl.pop(i - 1)

        return sl[-1][1]
```
