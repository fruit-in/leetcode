# 1237. 找出给定方程的正整数解
给出一个函数  ```f(x, y)``` 和一个目标结果 ```z```，请你计算方程 ```f(x,y) == z``` 所有可能的正整数 **数对** ```x``` 和 ```y```。

给定函数是严格单调的，也就是说：
* ```f(x, y) < f(x + 1, y)```
* ```f(x, y) < f(x, y + 1)```

函数接口定义如下：
```
interface CustomFunction {
public:
  // Returns positive integer f(x, y) for any given positive integer x and y.
  int f(int x, int y);
};
```

如果你想自定义测试，你可以输入整数 ```function_id``` 和一个目标结果 ```z``` 作为输入，其中 ```function_id``` 表示一个隐藏函数列表中的一个函数编号，题目只会告诉你列表中的 ```2``` 个函数。

你可以将满足条件的 **结果数对** 按任意顺序返回。

#### 示例 1:
<pre>
<strong>输入:</strong> function_id = 1, z = 5
<strong>输出:</strong> [[1,4],[2,3],[3,2],[4,1]]
<strong>解释:</strong> function_id = 1 表示 f(x, y) = x + y
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> function_id = 2, z = 5
<strong>输出:</strong> [[1,5],[5,1]]
<strong>解释:</strong> function_id = 2 表示 f(x, y) = x * y
</pre>

#### 提示:
* ```1 <= function_id <= 9```
* ```1 <= z <= 100```
* 题目保证 ```f(x, y) == z``` 的解处于 ```1 <= x, y <= 1000``` 的范围内。
* 在 ```1 <= x, y <= 1000``` 的前提下，题目保证 ```f(x, y)``` 是一个 32 位有符号整数。

## 题解 (Python)

### 1. 暴力法
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

### 2. 二分查找
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

### 3. 双指针
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
