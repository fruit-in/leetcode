class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ''
        if strs:
            maxlength = len(strs[0])
            index = maxlength
            while len(prefix) != index:
                for s in strs[1:]:
                    if not s.startswith(strs[0][:index]):
                        maxlength = index
                        index = maxlength // 2
                        break
                else:
                    prefix = strs[0][:index]
                    index = (index + maxlength) // 2
        return prefix
