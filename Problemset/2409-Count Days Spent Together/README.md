# 2409. Count Days Spent Together
Alice and Bob are traveling to Rome for separate business meetings.

You are given 4 strings `arriveAlice`, `leaveAlice`, `arriveBob`, and `leaveBob`. Alice will be in the city from the dates `arriveAlice` to `leaveAlice` (**inclusive**), while Bob will be in the city from the dates `arriveBob` to `leaveBob` (**inclusive**). Each will be a 5-character string in the format `"MM-DD"`, corresponding to the month and day of the date.

Return *the total number of days that Alice and Bob are in Rome together*.

You can assume that all dates occur in the **same** calendar year, which is **not** a leap year. Note that the number of days per month can be represented as: `[31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]`.

#### Example 1:
<pre>
<strong>Input:</strong> arriveAlice = "08-15", leaveAlice = "08-18", arriveBob = "08-16", leaveBob = "08-19"
<strong>Output:</strong> 3
<strong>Explanation:</strong> Alice will be in Rome from August 15 to August 18. Bob will be in Rome from August 16 to August 19. They are both in Rome together on August 16th, 17th, and 18th, so the answer is 3.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arriveAlice = "10-01", leaveAlice = "10-31", arriveBob = "11-01", leaveBob = "12-31"
<strong>Output:</strong> 0
<strong>Explanation:</strong> There is no day when Alice and Bob are in Rome together, so we return 0.
</pre>

#### Constraints:
* All dates are provided in the format `"MM-DD"`.
* Alice and Bob's arrival dates are **earlier than or equal to** their leaving dates.
* The given dates are valid dates of a **non-leap** year.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countDaysTogether(self, arriveAlice: str, leaveAlice: str, arriveBob: str, leaveBob: str) -> int:
        days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334]
        arriveAliceMM, arriveAliceDD = int(
            arriveAlice[:2]), int(arriveAlice[3:])
        leaveAliceMM, leaveAliceDD = int(leaveAlice[:2]), int(leaveAlice[3:])
        arriveBobMM, arriveBobDD = int(arriveBob[:2]), int(arriveBob[3:])
        leaveBobMM, leaveBobDD = int(leaveBob[:2]), int(leaveBob[3:])
        arriveAliceDay = days[arriveAliceMM - 1] + arriveAliceDD
        leaveAliceDay = days[leaveAliceMM - 1] + leaveAliceDD
        arriveBobDay = days[arriveBobMM - 1] + arriveBobDD
        leaveBobDay = days[leaveBobMM - 1] + leaveBobDD
        ret = min(leaveAliceDay, leaveBobDay) - \
            max(arriveAliceDay, arriveBobDay) + 1

        return max(ret, 0)
```
