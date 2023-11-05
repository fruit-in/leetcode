class MapSum:

    def __init__(self):
        self.keyval = {}
        self.trie = {}

    def insert(self, key: str, val: int) -> None:
        diff = val - self.keyval.get(key, 0)
        self.keyval[key] = val
        curr = self.trie

        for ch in key:
            if ch not in curr:
                curr[ch] = {"val": 0}
            curr = curr[ch]
            curr["val"] += diff

    def sum(self, prefix: str) -> int:
        curr = self.trie

        for i, ch in enumerate(prefix):
            if ch not in curr:
                return 0
            curr = curr[ch]
            if i == len(prefix) - 1:
                return curr["val"]


# Your MapSum object will be instantiated and called as such:
# obj = MapSum()
# obj.insert(key,val)
# param_2 = obj.sum(prefix)
