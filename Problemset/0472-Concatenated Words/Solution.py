class TrieNode:
    def __init__(self):
        self.children = {}
        self.isend = False


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word: str) -> None:
        curr = self.root

        for i in range(len(word)):
            if word[i] not in curr.children:
                curr.children[word[i]] = TrieNode()
            curr = curr.children[word[i]]

        curr.isend = True

    def search(self, word: str) -> bool:
        n = len(word)
        dp = [False] * (n + 1)
        dp[0] = True

        for i in range(n):
            if not dp[i]:
                continue

            curr = self.root

            for j in range(i, n):
                if word[j] not in curr.children:
                    break
                curr = curr.children[word[j]]
                dp[j + 1] |= curr.isend

        return dp[n]


class Solution:
    def findAllConcatenatedWordsInADict(self, words: List[str]) -> List[str]:
        trie = Trie()
        ret = []

        for word in sorted(words, key=len):
            if trie.search(word):
                ret.append(word)
            else:
                trie.insert(word)

        return ret
