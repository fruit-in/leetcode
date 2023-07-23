# 2446. Determine if Two Events Have Conflict
You are given two arrays of strings that represent two inclusive events that happened **on the same day**, `event1` and `event2`, where:

* <code>event1 = [startTime<sub>1</sub>, endTime<sub>1</sub>]</code> and
* <code>event2 = [startTime<sub>2</sub>, endTime<sub>2</sub>]</code>.

Event times are valid 24 hours format in the form of `HH:MM`.

A **conflict** happens when two events have some non-empty intersection (i.e., some moment is common to both events).

Return `true` *if there is a conflict between two events. Otherwise, return* `false`.

#### Example 1:
<pre>
<strong>Input:</strong> event1 = ["01:15","02:00"], event2 = ["02:00","03:00"]
<strong>Output:</strong> true
<strong>Explanation:</strong> The two events intersect at time 2:00.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> event1 = ["01:00","02:00"], event2 = ["01:20","03:00"]
<strong>Output:</strong> true
<strong>Explanation:</strong> The two events intersect starting from 01:20 to 02:00.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> event1 = ["10:00","11:00"], event2 = ["14:00","15:00"]
<strong>Output:</strong> false
<strong>Explanation:</strong> The two events do not intersect.
</pre>

#### Constraints:
* `evnet1.length == event2.length == 2.`
* `event1[i].length == event2[i].length == 5`
* <code>startTime<sub>1</sub> <= endTime<sub>1</sub></code>
* <code>startTime<sub>2</sub> <= endTime<sub>2</sub></code>
* All the event times follow the `HH:MM` format.

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def haveConflict(self, event1: List[str], event2: List[str]) -> bool:
        start1 = int(event1[0][:2]) * 60 + int(event1[0][3:])
        end1 = int(event1[1][:2]) * 60 + int(event1[1][3:])
        start2 = int(event2[0][:2]) * 60 + int(event2[0][3:])
        end2 = int(event2[1][:2]) * 60 + int(event2[1][3:])

        return (end1 >= start2 and end1 <= end2) or (end2 >= start1 and end2 <= end1)
```
