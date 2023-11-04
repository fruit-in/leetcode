class Solution:
    def replaceWords(self, dictionary: List[str], sentence: str) -> str:
        trie = {}
        words = sentence.split()

        for root in dictionary:
            curr = trie

            for i in range(len(root)):
                if root[i] not in curr:
                    curr[root[i]] = {}
                curr = curr[root[i]]
                if i == len(root) - 1:
                    curr[''] = {}

        for i in range(len(words)):
            curr = trie

            for j in range(len(words[i])):
                if words[i][j] not in curr:
                    break
                curr = curr[words[i][j]]
                if '' in curr:
                    words[i] = words[i][:j + 1]
                    break

        return ' '.join(words)
