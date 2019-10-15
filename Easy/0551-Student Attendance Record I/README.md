# 551. Student Attendance Record I
You are given a string representing an attendance record for a student. The record only contains the following three characters: 
1. **'A'** : Absent. 
2. **'L'** : Late.
3. **'P'** : Present. 

A student could be rewarded if his attendance record doesn't contain **more than one 'A' (absent)** or **more than two continuous 'L' (late)**.

You need to return whether the student could be rewarded according to his attendance record.

#### Example 1:
<pre>
<strong>Input:</strong> "PPALLP"
<strong>Output:</strong> True
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> "PPALLL"
<strong>Output:</strong> False
</pre>

## Solutions (Python)

### 1. Solution
```Python3
class Solution:
    def checkRecord(self, s: str) -> bool:
        return s.count('A') < 2 and s.count('LLL') < 1
```
