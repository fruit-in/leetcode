class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        row = [1] * (rowIndex + 1)
        for i in range(2, rowIndex + 1):
            prenum = 1
            for j in range(1, i):
                row[j], prenum = row[j] + prenum, row[j]
            row[j + 1] = 1
        return row
