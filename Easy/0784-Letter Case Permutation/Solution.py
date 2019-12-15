class Solution:
    def letterCasePermutation(self, S: str) -> List[str]:
        ret = [""]

        for ch in S:
            if ch.isalpha():
                tmp = [s + ch.lower() for s in ret]
                ret = [s + ch.upper() for s in ret]
                ret.extend(tmp)
            else:
                ret = [s + ch for s in ret]

        return ret
