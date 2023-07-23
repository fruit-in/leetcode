# 326. 3的幂
给定一个整数，写一个函数来判断它是否是 3 的幂次方。

#### 示例 1:
<pre>
<strong>输入:</strong> 27
<strong>输出:</strong> true
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 0
<strong>输出:</strong> false
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> 9
<strong>输出:</strong> true
</pre>

#### 示例 4:
<pre>
<strong>输入:</strong> 45
<strong>输出:</strong> false
</pre>

#### 进阶:
你能不使用循环或者递归来完成本题吗？

## 题解 (Python)

### 1. 迭代
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        while n % 3 == 0 and n != 0:
            n //= 3

        return n == 1
```

### 2. 数学
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        return n > 0 and 3 ** round(math.log(n, 3)) == n
```

### 3. 整数限制
```Python3
class Solution:
    def isPowerOfThree(self, n: int) -> bool:
        return n > 0 and 1162261467 % n == 0
```
