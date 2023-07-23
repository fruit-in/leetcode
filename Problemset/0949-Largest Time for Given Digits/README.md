# 949. Largest Time for Given Digits
Given an array of 4 digits, return the largest 24 hour time that can be made.

The smallest 24 hour time is 00:00, and the largest is 23:59.  Starting from 00:00, a time is larger if more time has elapsed since midnight.

Return the answer as a string of length 5.  If no valid time can be made, return an empty string.

#### Example 1:
<pre>
<strong>Input:</strong> [1,2,3,4]
<strong>Output:</strong> "23:41"
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> [5,5,5,5]
<strong>Output:</strong> ""
</pre>

#### Note:
1. ```A.length == 4```
2. ```0 <= A[i] <= 9```

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def largestTimeFromDigits(self, A: List[int]) -> str:
        cnt0_3 = sum(1 if x < 4 else 0 for x in A)
        cnt0_5 = sum(1 if x < 6 else 0 for x in A)
        if 2 in A and cnt0_3 > 1 and cnt0_5 > 2:
            A.remove(2)
            hour = "2" + str(max(filter(lambda x: x < 4, A)))
            A.remove(max(filter(lambda x: x < 4, A)))
        elif 1 in A:
            A.remove(1)
            hour = "1" + str(max(A))
            A.remove(max(A))
        elif 0 in A:
            A.remove(0)
            hour = "0" + str(max(A))
            A.remove(max(A))
        else:
            return ""

        if max(A) < 6:
            return hour + ":" + str(max(A)) + str(min(A))
        elif min(A) < 6:
            return hour + ":" + str(min(A)) + str(max(A))
        else:
            return ""
```
