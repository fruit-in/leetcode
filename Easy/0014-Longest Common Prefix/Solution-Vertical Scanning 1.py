class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ''
        if strs:
            for i in range(len(strs[0])):
                temp = strs[0][:i + 1]
                for s in strs[1:]:
                    if not s.startswith(temp):
                        return prefix
                prefix = temp
        return prefix
