class Solution:
    def divideString(self, s: str, k: int, fill: str) -> List[str]:
        ret = [s[i:i + k] for i in range(0, len(s), k)]
        ret[-1] += (k - len(ret[-1])) * fill

        return ret
