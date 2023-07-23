# 303. 区域和检索 - 数组不可变
给定一个整数数组  *nums*，求出数组从索引 *i* 到 *j*  (*i* ≤ *j*) 范围内元素的总和，包含 *i*,  *j* 两点。

#### 示例:
```
给定 nums = [-2, 0, 3, -5, 2, -1]，求和函数为 sumRange()

sumRange(0, 2) -> 1
sumRange(2, 5) -> -1
sumRange(0, 5) -> -3
```

#### 说明:
1. 你可以假设数组不可变。
2. 会多次调用 *sumRange* 方法。

## 题解 (Python)

### 1. 暴力法
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

### 2. 缓存
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
