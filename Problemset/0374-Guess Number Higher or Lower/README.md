# 374. Guess Number Higher or Lower
We are playing the Guess Game. The game is as follows:

I pick a number from **1** to ***n***. You have to guess which number I picked.

Every time you guess wrong, I'll tell you whether the number is higher or lower.

You call a pre-defined API ```guess(int num)``` which returns 3 possible results (```-1```, ```1```, or ```0```):
```
-1 : My number is lower
 1 : My number is higher
 0 : Congrats! You got it!
```

#### Example:
<pre>
<strong>Input:</strong> n = 10, pick = 6
<strong>Output:</strong> 6
</pre>

## Solutions (Python)

### 1. Binary Search
```Python
# The guess API is already defined for you.
# @param num, your guess
# @return -1 if my number is lower, 1 if my number is higher, otherwise return 0
# def guess(num):

class Solution(object):
    def guessNumber(self, n):
        """
        :type n: int
        :rtype: int
        """
        l, r = 1, n
        while l <= r:
            m = (l + r) // 2
            res = guess(m)
            if res == 0:
                return m
            elif res == 1:
                l = m + 1
            elif res == -1:
                r = m - 1
```
