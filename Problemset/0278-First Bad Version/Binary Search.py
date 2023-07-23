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
