# 1626. Best Team With No Conflicts
You are the manager of a basketball team. For the upcoming tournament, you want to choose the team with the highest overall score. The score of the team is the **sum** of scores of all the players in the team.

However, the basketball team is not allowed to have **conflicts**. A **conflict** exists if a younger player has a **strictly higher** score than an older player. A conflict does **not** occur between players of the same age.

Given two lists, `scores` and `ages`, where each `scores[i]` and `ages[i]` represents the score and age of the <code>i<sup>th</sup></code> player, respectively, return *the highest overall score of all possible basketball teams*.

#### Example 1:
<pre>
<strong>Input:</strong> scores = [1,3,5,10,15], ages = [1,2,3,4,5]
<strong>Output:</strong> 34
<strong>Explanation:</strong> You can choose all the players.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> scores = [4,5,6,5], ages = [2,1,2,1]
<strong>Output:</strong> 16
<strong>Explanation:</strong> It is best to choose the last 3 players. Notice that you are allowed to choose multiple people of the same age.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> scores = [1,2,3,5], ages = [8,9,10,1]
<strong>Output:</strong> 6
<strong>Explanation:</strong> It is best to choose the first 3 players.
</pre>

#### Constraints:
* `1 <= scores.length, ages.length <= 1000`
* `scores.length == ages.length`
* <code>1 <= scores[i] <= 10<sup>6</sup></code>
* `1 <= ages[i] <= 1000`

## Solutions (Python)

### 1. Solution
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
