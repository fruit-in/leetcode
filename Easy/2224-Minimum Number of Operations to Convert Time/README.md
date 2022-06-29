# 2224. Minimum Number of Operations to Convert Time
You are given two strings `current` and `correct` representing two **24-hour times**.

24-hour times are formatted as `"HH:MM"`, where `HH` is between `00` and `23`, and `MM` is between `00` and `59`. The earliest 24-hour time is `00:00`, and the latest is `23:59`.

In one operation you can increase the time `current` by `1`, `5`, `15`, or `60` minutes. You can perform this operation **any** number of times.

Return *the **minimum number of operations** needed to convert* `current` *to* `correct`.

#### Example 1:
<pre>
<strong>Input:</strong> current = "02:30", correct = "04:35"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
We can convert current to correct in 3 operations as follows:
- Add 60 minutes to current. current becomes "03:30".
- Add 60 minutes to current. current becomes "04:30".
- Add 5 minutes to current. current becomes "04:35".
It can be proven that it is not possible to convert current to correct in fewer than 3 operations.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> current = "11:00", correct = "11:01"
<strong>Output:</strong> 1
<strong>Explanation:</strong> We only have to add one minute to current, so the minimum number of operations needed is 1.
</pre>

#### Constraints:
* `current` and `correct` are in the format `"HH:MM"`
* `current <= correct`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def convertTime(self, current: str, correct: str) -> int:
        current = int(current[:2]) * 60 + int(current[3:])
        correct = int(correct[:2]) * 60 + int(correct[3:])
        diff = correct - current + (1440 if correct < current else 0)
        ret = 0

        ret, diff = ret + diff // 60,  diff % 60
        ret, diff = ret + diff // 15,  diff % 15
        ret, diff = ret + diff // 5,  diff % 5
        ret += diff

        return ret
```
