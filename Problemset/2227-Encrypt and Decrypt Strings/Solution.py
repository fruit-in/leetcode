class Encrypter:

    def __init__(self, keys: List[str], values: List[str], dictionary: List[str]):
        self.encryptmap = dict(zip(keys, values))
        self.count = {}
        keys = set(keys)

        for word in dictionary:
            if all(c in keys for c in word):
                s = ''.join(self.encryptmap[c] for c in word)
                if s not in self.count:
                    self.count[s] = 0
                self.count[s] += 1

    def encrypt(self, word1: str) -> str:
        return ''.join(self.encryptmap[c] for c in word1)

    def decrypt(self, word2: str) -> int:
        return self.count.get(word2, 0)


# Your Encrypter object will be instantiated and called as such:
# obj = Encrypter(keys, values, dictionary)
# param_1 = obj.encrypt(word1)
# param_2 = obj.decrypt(word2)
