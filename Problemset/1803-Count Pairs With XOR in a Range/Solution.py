class Trie:
    def __init__(self, maxheight: int, height: int):
        self.maxheight = maxheight
        self.height = height
        self.children = [None, None]
        self.count = 0

    def get(self, num: int, limit: int) -> int:
        if self.height == 0:
            return self.count

        ret = 0

        for i in range(2):
            if self.children[i]:
                num ^= (i << (self.height - 1))
                minxor = num & ~((1 << (self.height - 1)) - 1)
                maxxor = num | ((1 << (self.height - 1)) - 1)

                if maxxor <= limit:
                    ret += self.children[i].count
                elif minxor <= limit:
                    ret += self.children[i].get(num, limit)

        return ret

    def insert(self, num: int) -> None:
        self.count += 1

        if self.height > 0:
            i = (num >> (self.height - 1)) & 1
            if not self.children[i]:
                self.children[i] = Trie(self.maxheight, self.height - 1)
            self.children[i].insert(num)


class Solution:
    def countPairs(self, nums: List[int], low: int, high: int) -> int:
        maxheight = int(log2(max(high, max(nums)))) + 1
        trie = Trie(maxheight, maxheight)
        ret = 0

        for num in nums:
            ret += trie.get(num, high) - trie.get(num, low - 1)
            trie.insert(num)

        return ret
