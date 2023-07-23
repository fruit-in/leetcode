# 398. Random Pick Index
Given an array of integers with possible duplicates, randomly output the index of a given target number. You can assume that the given target number must exist in the array.

#### Note:
The array size can be very large. Solution that uses too much extra space will not pass the judge.

#### Example:
```
int[] nums = new int[] {1,2,3,3,3};
Solution solution = new Solution(nums);

// pick(3) should return either index 2, 3, or 4 randomly. Each index should have equal probability of returning.
solution.pick(3);

// pick(1) should return 0. Since in the array only nums[0] is equal to 1.
solution.pick(1);
```

## Solutions (Python)

### 1. HashMap
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

### 2. Reservoir Sampling
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
