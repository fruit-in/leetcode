# 1399. Count Largest Group
Given an integer ```n```. Each number from ```1``` to ```n``` is grouped according to the sum of its digits.

Return how many groups have the largest size.

#### Example 1:
<pre>
<strong>Input:</strong> n = 13
<strong>Output:</strong> 4
<strong>Explanation:</strong> There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
[1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9]. There are 4 groups with largest size.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> 2
<strong>Explanation:</strong> There are 2 groups [1], [2] of size 1.
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 15
<strong>Output:</strong> 6
</pre>

#### Example 4:
<pre>
<strong>Input:</strong> n = 24
<strong>Output:</strong> 5
</pre>

#### Constraints:
* ```1 <= n <= 10^4```

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def countLargestGroup(self, n: int) -> int:
        counter = {}

        for i in range(1, n + 1):
            digits_sum = sum(map(int, str(i)))
            if digits_sum not in counter:
                counter[digits_sum] = 0
            counter[digits_sum] += 1

        largest_size = max(counter.values())

        return sum(map(lambda x: x == largest_size, counter.values()))
```
