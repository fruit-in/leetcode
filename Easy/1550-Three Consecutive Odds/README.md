# 1550. Three Consecutive Odds
Given an integer array `arr`, return `true` if there are three consecutive odd numbers in the array. Otherwise, return `false`. 

#### Example 1:
<pre>
<strong>Input:</strong> arr = [2,6,4,1]
<strong>Output:</strong> false
<strong>Explanation:</strong> There are no three consecutive odds.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> arr = [1,2,34,3,4,5,7,23,12]
<strong>Output:</strong> true
<strong>Explanation:</strong> [5,7,23] are three consecutive odds.
</pre>

#### Constraints:
* `1 <= arr.length <= 1000`
* `1 <= arr[i] <= 1000`

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def threeConsecutiveOdds(self, arr: List[int]) -> bool:
        for i in range(len(arr) - 2):
            if arr[i] % 2 == arr[i + 1] % 2 == arr[i + 2] % 2 == 1:
                return True

        return False
```
