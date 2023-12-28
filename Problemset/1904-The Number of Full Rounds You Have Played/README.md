# 1904. The Number of Full Rounds You Have Played
You are participating in an online chess tournament. There is a chess round that starts every `15` minutes. The first round of the day starts at `00:00`, and after every `15` minutes, a new round starts.

* For example, the second round starts at `00:15`, the fourth round starts at `00:45`, and the seventh round starts at `01:30`.

You are given two strings `loginTime` and `logoutTime` where:

* `loginTime` is the time you will login to the game, and
* `logoutTime` is the time you will logout from the game.

If `logoutTime` is **earlier** than `loginTime`, this means you have played from `loginTime` to midnight and from midnight to `logoutTime`.

Return *the number of full chess rounds you have played in the tournament*.

**Note:** All the given times follow the 24-hour clock. That means the first round of the day starts at `00:00` and the last round of the day starts at `23:45`.

#### Example 1:
<pre>
<strong>Input:</strong> loginTime = "09:31", logoutTime = "10:14"
<strong>Output:</strong> 1
<strong>Explanation:</strong> You played one full round from 09:45 to 10:00.
You did not play the full round from 09:30 to 09:45 because you logged in at 09:31 after it began.
You did not play the full round from 10:00 to 10:15 because you logged out at 10:14 before it ended.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> loginTime = "21:30", logoutTime = "03:00"
<strong>Output:</strong> 22
<strong>Explanation:</strong> You played 10 full rounds from 21:30 to 00:00 and 12 full rounds from 00:00 to 03:00.
10 + 12 = 22.
</pre>

#### Constraints:
* `loginTime` and `logoutTime` are in the format `hh:mm`.
* `00 <= hh <= 23`
* `00 <= mm <= 59`
* `loginTime` and `logoutTime` are not equal.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def numberOfRounds(self, loginTime: str, logoutTime: str) -> int:
        loginhour = int(loginTime[:2])
        loginmin = int(loginTime[3:])
        logouthour = int(logoutTime[:2]) + \
            (24 if logoutTime < loginTime else 0)
        logoutmin = int(logoutTime[3:])
        ret = (logouthour + 1 - loginhour) * 4

        if loginmin > 45:
            ret -= 4
        elif loginmin > 30:
            ret -= 3
        elif loginmin > 15:
            ret -= 2
        elif loginmin > 0:
            ret -= 1
        if logoutmin < 15:
            ret -= 4
        elif logoutmin < 30:
            ret -= 3
        elif logoutmin < 45:
            ret -= 2
        else:
            ret -= 1

        return max(ret, 0)
```
