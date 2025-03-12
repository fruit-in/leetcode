from functools import cache


class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> List[str]:
        words = set(wordDict)

        @cache
        def backtracking(s: str) -> Optional[List[str]]:
            ret = []

            for i in range(1, min(len(s) + 1, 10)):
                if s[:i] in wordDict:
                    if i == len(s):
                        return ret + [s]
                    sentences = backtracking(s[i:])
                    if sentences is not None:
                        ret.extend("{} {}".format(s[:i], sentence)
                                   for sentence in sentences)

            return ret if ret != [] else None

        return backtracking(s) if backtracking(s) is not None else []
