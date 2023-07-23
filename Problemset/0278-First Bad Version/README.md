# 278. First Bad Version
You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.

Suppose you have ```n``` versions ```[1, 2, ..., n]``` and you want to find out the first bad one, which causes all the following ones to be bad.

You are given an API ```bool isBadVersion(version)``` which will return whether ```version``` is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.

#### Example:
<pre>
Given n = 5, and version = 4 is the first bad version.

call isBadVersion(3) -> false
call isBadVersion(5) -> true
call isBadVersion(4) -> true

Then 4 is the first bad version.
</pre>

## Solutions (Python)

### 1. Binary Search
```Python3
# The isBadVersion API is already defined for you.
# @param version, an integer
# @return a bool
# def isBadVersion(version):

class Solution:
    def firstBadVersion(self, n):
        """
        :type n: int
        :rtype: int
        """
        l, r = 1, n
        while l <= r:
            m = (l + r) // 2
            if isBadVersion(m) and (m == 1 or not isBadVersion(m - 1)):
                return m
            elif not isBadVersion(m):
                l = m + 1
            else:
                r = m - 1
```
