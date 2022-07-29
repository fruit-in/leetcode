# 2225. 找出输掉零场或一场比赛的玩家
给你一个整数数组 `matches` 其中 `matches[i] = [winneri, loseri]` 表示在一场比赛中 `winneri` 击败了 `loseri` 。

返回一个长度为 2 的列表 `answer` ：
* `answer[0]` 是所有 **没有** 输掉任何比赛的玩家列表。
* `answer[1]` 是所有恰好输掉 **一场** 比赛的玩家列表。

两个列表中的值都应该按 **递增** 顺序返回。

**注意**：
* 只考虑那些参与 **至少一场** 比赛的玩家。
* 生成的测试用例保证 **不存在** 两场比赛结果 **相同** 。

#### 示例 1:
<pre>
<strong>输入:</strong> matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
<strong>输出:</strong> [[1,2,10],[4,5,7,8]]
<strong>解释:</strong>
玩家 1、2 和 10 都没有输掉任何比赛。
玩家 4、5、7 和 8 每个都输掉一场比赛。
玩家 3、6 和 9 每个都输掉两场比赛。
因此，answer[0] = [1,2,10] 和 answer[1] = [4,5,7,8] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> matches = [[2,3],[1,3],[5,4],[6,4]]
<strong>输出:</strong> [[1,2,5,6],[]]
<strong>解释:</strong>
玩家 1、2、5 和 6 都没有输掉任何比赛。
玩家 3 和 4 每个都输掉两场比赛。
因此，answer[0] = [1,2,5,6] 和 answer[1] = [] 。
</pre>

#### 提示:
* <code>1 <= matches.length <= 10<sup>5</sup></code>
* `matches[i].length == 2`
* <code>1 <= winneri, loseri <= 10<sup>5</sup></code>
* `winneri != loseri`
* 所有 `matches[i]` **互不相同**

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findWinners(self, matches: List[List[int]]) -> List[List[int]]:
        count = {}
        answer = []

        for winner, loser in matches:
            if winner not in count:
                count[winner] = 0
            if loser not in count:
                count[loser] = 0
            count[loser] += 1

        answer.append(sorted(k for k, v in count.items() if v == 0))
        answer.append(sorted(k for k, v in count.items() if v == 1))

        return answer
```
