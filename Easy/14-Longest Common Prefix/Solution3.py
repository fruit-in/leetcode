class Solution:
    def longestCommonPrefix(self, strs):
        if len(strs) == 0:
            return ''
        elif len(strs) == 1:
            return strs[0]
        else:
            prefix1 = self.longestCommonPrefix(strs[:len(strs) // 2])
            prefix2 = self.longestCommonPrefix(strs[len(strs) // 2:])
            for i in range(len(prefix1) if len(prefix1) < len(prefix2) else len(prefix2)):
                if prefix1[i] != prefix2[i]:
                    return prefix1[:i]
            return prefix1[:len(prefix2)]
