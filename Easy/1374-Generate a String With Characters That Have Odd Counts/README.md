# 1374. Generate a String With Characters That Have Odd Counts
Given an integer ```n```, *return a string with ```n``` characters such that each character in such string occurs **an odd number of times***.

The returned string must contain only lowercase English letters. If there are multiples valid strings, return **any** of them.

#### Example 1:
<pre>
<strong>Input:</strong> n = 4
<strong>Output:</strong> "pppz"
<strong>Explanation:</strong> "pppz" is a valid string since the character 'p' occurs three times and the character 'z' occurs once. Note that there are many other valid strings such as "ohhh" and "love".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> n = 2
<strong>Output:</strong> "xy"
<strong>Explanation:</strong> "xy" is a valid string since the characters 'x' and 'y' occur once. Note that there are many other valid strings such as "ag" and "ur".
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> n = 7
<strong>Output:</strong> "holasss"
</pre>

#### Constraints:
* ```1 <= n <= 500```

## Solutions (Python)

### 1. Solution
```Python
class Solution:
    def generateTheString(self, n: int) -> str:
        return 'a' * (n - 1 + n % 2) + 'b' * ((n + 1) % 2)
```
