class Solution:
    def numberOfLines(self, widths: List[int], S: str) -> List[int]:
        ret = [1, 0]
        for ch in S:
            if ret[1] + widths[ord(ch) - 97] > 100:
                ret[0] += 1
                ret[1] = 0
            ret[1] += widths[ord(ch) - 97]

        return ret
