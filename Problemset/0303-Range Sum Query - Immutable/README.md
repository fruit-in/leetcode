# 303. Range Sum Query - Immutable
Given an integer array *nums*, find the sum of the elements between indices *i* and *j* (*i* ≤ *j*), inclusive.

#### Example:
```
Given nums = [-2, 0, 3, -5, 2, -1]

sumRange(0, 2) -> 1
sumRange(2, 5) -> -1
sumRange(0, 5) -> -3
```

#### Note:
1. You may assume that the array does not change.
2. There are many calls to *sumRange* function.

## Solutions (Python)

### 1. Brute Force
```Python3
class NumArray:

    def __init__(self, nums: List[int]):
        self.data = nums

    def sumRange(self, i: int, j: int) -> int:
        return sum(self.data[i: j + 1])


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# param_1 = obj.sumRange(i,j)
```

### 2. Caching
```Python3
class NumArray:

    def __init__(self, nums: List[int]):
        self.cache = [0]
        for n in nums:
            self.cache.append(self.cache[-1] + n)

    def sumRange(self, i: int, j: int) -> int:
        return self.cache[j + 1] - self.cache[i]


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# param_1 = obj.sumRange(i,j)
```
