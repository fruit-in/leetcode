class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        if not strs:
            return ''
        return self.findPrefix(strs[1:], strs[0])

    def findPrefix(self, strs: List[str], prefix: str) -> str:
        for k, v in enumerate(strs):
            if not v.startswith(prefix):
                return self.findPrefix(strs[k:], prefix[:-1])
        return prefix
