class NumArray:

    def __init__(self, nums: List[int]):
        self.tree = [0] * (len(nums) + 1)

        for i, x in enumerate(nums):
            self.update(i, x)

    def update(self, index: int, val: int) -> None:
        val -= self.sumRange(index, index)
        index += 1

        while index < len(self.tree):
            self.tree[index] += val
            index += index & -index

    def sumRange(self, left: int, right: int) -> int:
        right += 1
        ret = 0

        while right > 0:
            ret += self.tree[right]
            right -= right & -right
        while left > 0:
            ret -= self.tree[left]
            left -= left & -left

        return ret


# Your NumArray object will be instantiated and called as such:
# obj = NumArray(nums)
# obj.update(index,val)
# param_2 = obj.sumRange(left,right)
