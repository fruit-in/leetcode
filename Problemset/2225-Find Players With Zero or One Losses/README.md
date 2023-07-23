# 2225. Find Players With Zero or One Losses
You are given an integer array `matches` where `matches[i] = [winneri, loseri]` indicates that the player `winneri` defeated player `loseri` in a match.

Return *a list* `answer` *of size* `2` *where*:
* `answer[0]` is a list of all players that have **not** lost any matches.
* `answer[1]` is a list of all players that have lost exactly **one** match.

The values in the two lists should be returned in **increasing** order.

**Note:**
* You should only consider the players that have played **at least one** match.
* The testcases will be generated such that no two matches will have the **same** outcome.

#### Example 1:
<pre>
<strong>Input:</strong> matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
<strong>Output:</strong> [[1,2,10],[4,5,7,8]]
<strong>Explanation:</strong>
Players 1, 2, and 10 have not lost any matches.
Players 4, 5, 7, and 8 each have lost one match.
Players 3, 6, and 9 each have lost two matches.
Thus, answer[0] = [1,2,10] and answer[1] = [4,5,7,8].
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> matches = [[2,3],[1,3],[5,4],[6,4]]
<strong>Output:</strong> [[1,2,5,6],[]]
<strong>Explanation:</strong>
Players 1, 2, 5, and 6 have not lost any matches.
Players 3 and 4 each have lost two matches.
Thus, answer[0] = [1,2,5,6] and answer[1] = [].
</pre>

#### Constraints:
* <code>1 <= matches.length <= 10<sup>5</sup></code>
* `matches[i].length == 2`
* <code>1 <= winneri, loseri <= 10<sup>5</sup></code>
* `winneri != loseri`
* All `matches[i]` are **unique**.

## Solutions (Python)

### 1. Solution
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
