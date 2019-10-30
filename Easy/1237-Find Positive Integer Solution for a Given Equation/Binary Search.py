"""
   This is the custom function interface.
   You should not implement it, or speculate about its implementation
   class CustomFunction:
       # Returns f(x, y) for any given positive integers x and y.
       # Note that f(x, y) is increasing with respect to both x and y.
       # i.e. f(x, y) < f(x + 1, y), f(x, y) < f(x, y + 1)
       def f(self, x, y):
  
"""
class Solution:
    def findSolution(self, customfunction: 'CustomFunction', z: int) -> List[List[int]]:
        ret = []
        min_x, max_x = 1, 1000

        l, r = 2, 1000
        while l <= r:
            m = (l + r) // 2
            if customfunction.f(m, 1000) < z:
                l = m + 1
            elif customfunction.f(m - 1, 1000) >= z:
                r = m - 1
            else:
                min_x = m
                break

        l, r = 1, 999
        while l <= r:
            m = (l + r) // 2
            if customfunction.f(m, 1) > z:
                r = m - 1
            elif customfunction.f(m + 1, 1) <= z:
                l = m + 1
            else:
                max_x = m
                break

        for x in range(min_x, max_x + 1):
            l, r = 1, 1000
            while l <= r:
                m = (l + r) // 2
                if customfunction.f(x, m) < z:
                    l = m + 1
                elif customfunction.f(x, m) > z:
                    r = m - 1
                else:
                    ret.append([x, m])
                    break

        return ret
