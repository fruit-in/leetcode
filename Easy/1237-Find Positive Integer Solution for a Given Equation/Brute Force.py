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

        max_y = 1000
        for x in range(1, 1001):
            if customfunction.f(x, 1000) < z:
                continue
            if customfunction.f(x, 1) > z:
                break

            for y in range(1, max_y + 1):
                if customfunction.f(x, y) > z:
                    max_y = y - 1
                    break
                elif customfunction.f(x, y) == z:
                    ret.append([x, y])
                    max_y = y - 1
                    break

        return ret
