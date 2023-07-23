class Solution:
    def oddString(self, words: List[str]) -> str:
        n = len(words[0])
        difference = []

        for i in range(len(words)):
            difference.append([])
            for j in range(n - 1):
                difference[i].append(ord(words[i][j + 1]) - ord(words[i][j]))

            if i >= 2:
                prev = difference[i - 1] == difference[i - 2]
                curr = difference[i] == difference[i - 1]
                if prev and not curr:
                    return words[i]
                elif not prev and curr:
                    return words[i - 2]
                elif not prev and not curr:
                    return words[i - 1]
