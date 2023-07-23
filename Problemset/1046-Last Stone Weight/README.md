# 1046. Last Stone Weight
We have a collection of rocks, each rock has a positive integer weight.

Each turn, we choose the two **heaviest** rocks and smash them together.  Suppose the stones have weights <code>x</code> and <code>y</code> with <code>x <= y</code>.  The result of this smash is:
* If <code>x == y</code>, both stones are totally destroyed;
* If <code>x != y</code>, the stone of weight <code>x</code> is totally destroyed, and the stone of weight <code>y</code> has new weight <code>y-x</code>.

At the end, there is at most 1 stone left.  Return the weight of this stone (or 0 if there are no stones left.)

#### Example 1:
<pre>
<strong>Input:</strong> [2,7,4,1,8,1]
<strong>Output:</strong> 1
<strong>Explanation:</strong> 
We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of last stone.
</pre>

#### Note:
1. <code>1 <= stones.length <= 30</code>
2. <code>1 <= stones[i] <= 1000</code>

## Solutions (Python)

### 1. Solution
```Python3
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
```
