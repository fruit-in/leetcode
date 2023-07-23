class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        tri = []
        for i in range(numRows):
            row = [1]
            for j in range(i - 1):
                row.append(tri[-1][j] + tri[-1][j + 1])
            if i:
                row.append(1)
            tri.append(row)
        return tri
