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
