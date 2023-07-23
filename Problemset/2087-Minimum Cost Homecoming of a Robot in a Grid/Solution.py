class Solution:
    def minCost(self, startPos: List[int], homePos: List[int], rowCosts: List[int], colCosts: List[int]) -> int:
        rowDir = 1 if homePos[0] < startPos[0] else - 1
        colDir = 1 if homePos[1] < startPos[1] else - 1
        rowSum = sum(rowCosts[homePos[0]:startPos[0]:rowDir])
        colSum = sum(colCosts[homePos[1]:startPos[1]:colDir])

        return rowSum + colSum
