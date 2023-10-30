class Solution:
    def findMaximumXOR(self, nums: List[int]) -> int:
        trie = [[], []]
        ret = 0

        for num in nums:
            curr = trie
            for i in range(30, -1, -1):
                j = (num >> i) & 1
                if curr[j] == []:
                    curr[j].append([])
                    curr[j].append([])
                curr = curr[j]

            curr = trie
            x = 0
            for i in range(30, -1, -1):
                j = (num >> i) & 1
                if curr[j ^ 1] != []:
                    j ^= 1
                curr = curr[j]
                x |= j << i

            ret = max(ret, x ^ num)

        return ret
