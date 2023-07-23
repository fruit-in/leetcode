class Solution:
    def maxArea(self, h: int, w: int, horizontalCuts: List[int], verticalCuts: List[int]) -> int:
        horizontalCuts.extend([0, h])
        verticalCuts.extend([0, w])
        horizontalCuts.sort()
        verticalCuts.sort()
        maxh = max(horizontalCuts[i] - horizontalCuts[i - 1]
                   for i in range(1, len(horizontalCuts)))
        maxw = max(verticalCuts[i] - verticalCuts[i - 1]
                   for i in range(1, len(verticalCuts)))

        return maxh * maxw % 1000000007
