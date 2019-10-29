# 1046. 最后一块石头的重量
有一堆石头，每块石头的重量都是正整数。

每一回合，从中选出两块**最重的**石头，然后将它们一起粉碎。假设石头的重量分别为 ```x``` 和 ```y```，且 ```x <= y```。那么粉碎的可能结果如下：
* 如果 ```x == y```，那么两块石头都会被完全粉碎；
* 如果 ```x != y```，那么重量为 ```x``` 的石头将会完全粉碎，而重量为 ```y``` 的石头新重量为 ```y-x```。

最后，最多只会剩下一块石头。返回此石头的重量。如果没有石头剩下，就返回 ```0```。

#### 提示:
1. ```1 <= stones.length <= 30```
2. ```1 <= stones[i] <= 1000```

## 题解 (Python)

### 1. 题解
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
