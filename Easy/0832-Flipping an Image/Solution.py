class Solution:
    def flipAndInvertImage(self, A: List[List[int]]) -> List[List[int]]:
        return [[0 if x == 1 else 1 for x in row] for row in map(reversed, A)]
