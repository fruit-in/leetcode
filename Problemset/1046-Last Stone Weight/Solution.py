class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        if len(stones) == 1:
            return stones.pop()
        if not stones:
            return 0
        stones.sort()
        new_stone = stones.pop() - stones.pop()
        if new_stone:
            stones.append(new_stone)
        return self.lastStoneWeight(stones)
