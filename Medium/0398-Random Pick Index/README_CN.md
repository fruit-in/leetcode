# 398. 随机数索引
给定一个可能含有重复元素的整数数组，要求随机输出给定的数字的索引。 您可以假设给定的数字一定存在于数组中。

#### 注意:
数组大小可能非常大。 使用太多额外空间的解决方案将不会通过测试。

#### 示例:
```
int[] nums = new int[] {1,2,3,3,3};
Solution solution = new Solution(nums);

// pick(3) 应该返回索引 2,3 或者 4。每个索引的返回概率应该相等。
solution.pick(3);

// pick(1) 应该返回 0。因为只有nums[0]等于1。
solution.pick(1);
```

## 题解 (Python)

### 1. 哈希表
```Python
class Solution:

    def __init__(self, nums: List[int]):
        self.indexes = {}

        for i, n in enumerate(nums):
            if n not in self.indexes:
                self.indexes[n] = []
            self.indexes[n].append(i)

    def pick(self, target: int) -> int:
        return random.choice(self.indexes[target])


# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.pick(target)
```

### 2. 蓄水池抽样
```Python
class Solution:

    def __init__(self, nums: List[int]):
        self.nums = nums

    def pick(self, target: int) -> int:
        cnt = 0
        ret = 0

        for i, n in enumerate(self.nums):
            if n == target:
                cnt += 1
                if random.randint(1, cnt) == cnt:
                    ret = i

        return ret


# Your Solution object will be instantiated and called as such:
# obj = Solution(nums)
# param_1 = obj.pick(target)
```
