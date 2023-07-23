class Solution:
    def restoreString(self, s: str, indices: List[int]) -> str:
        ret = [''] * len(s)

        for i in range(len(s)):
            ret[indices[i]] = s[i]

        return ''.join(ret)
