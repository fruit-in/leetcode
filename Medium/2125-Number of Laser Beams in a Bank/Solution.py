class Solution:
    def numberOfBeams(self, bank: List[str]) -> int:
        prev = 0
        ret = 0

        for row in bank:
            curr = row.count('1')
            ret += prev * curr
            if curr > 0:
                prev = curr

        return ret
