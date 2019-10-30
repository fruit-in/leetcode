# 1237. Find Positive Integer Solution for a Given Equation
Given a function  ```f(x, y)``` and a value ```z```, return all positive integer pairs ```x``` and ```y``` where ```f(x,y) == z```.

The function is constantly increasing, i.e.:
* ```f(x, y) < f(x + 1, y)```
* ```f(x, y) < f(x, y + 1)```

The function interface is defined like this:
```
interface CustomFunction {
public:
  // Returns positive integer f(x, y) for any given positive integer x and y.
  int f(int x, int y);
};
```

For custom testing purposes you're given an integer ```function_id``` and a target ```z``` as input, where ```function_id``` represent one function from an secret internal list, on the examples you'll know only two functions from the list.

You may return the solutions in any order.

#### Example 1:
<pre>
<strong>Input:</strong> function_id = 1, z = 5
<strong>Output:</strong> [[1,4],[2,3],[3,2],[4,1]]
<strong>Explanation:</strong> function_id = 1 means that f(x, y) = x + y
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> function_id = 2, z = 5
<strong>Output:</strong> [[1,5],[5,1]]
<strong>Explanation:</strong> function_id = 2 means that f(x, y) = x * y
</pre>

#### Constraints:
* ```1 <= function_id <= 9```
* ```1 <= z <= 100```
* It's guaranteed that the solutions of ```f(x, y) == z``` will be on the range ```1 <= x, y <= 1000```
* It's also guaranteed that ```f(x, y)``` will fit in 32 bit signed integer if ```1 <= x, y <= 1000```

## Solutions (Python)

### 1. Brute Force
```Python3
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
```

### 2. Binary Search
```Python3
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
```

### 3. Two Pointers
```Python3
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
        x, y = 1, 1000

        while x < 1001 and y > 0:
            if customfunction.f(x, y) > z:
                y -= 1
            elif customfunction.f(x, y) < z:
                x += 1
            else:
                ret.append([x, y])
                x += 1
                y -= 1

        return ret
```
