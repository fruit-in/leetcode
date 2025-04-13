class Solution:
    def sumPrefixScores(self, words: List[str]) -> List[int]:
        root = {}
        answer = [0] * len(words)

        for word in words:
            curr = root

            for c in word:
                if c not in curr:
                    curr[c] = [0, {}]
                curr[c][0] += 1
                curr = curr[c][1]

        for i, word in enumerate(words):
            curr = root

            for c in word:
                answer[i] += curr[c][0]
                curr = curr[c][1]

        return answer
