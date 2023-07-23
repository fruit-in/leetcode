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
