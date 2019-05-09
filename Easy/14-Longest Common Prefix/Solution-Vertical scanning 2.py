class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ''
        if strs:
            for i, c in enumerate(strs[0]):
                for s in strs[1:]:
                    if i == len(s) or s[i] != c:
                        return prefix
                prefix = strs[0][:i + 1]
        return prefix
