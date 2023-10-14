class Solution:
    def distinctNames(self, ideas: List[str]) -> int:
        firsts = [set() for _ in range(26)]
        ret = 0

        for idea in ideas:
            firsts[ord(idea[0]) - 97].add(idea[1:])

        for i in range(26):
            for j in range(i + 1, 26):
                intersectionlen = len(firsts[i] & firsts[j])
                ret += (len(firsts[i]) - intersectionlen) * \
                    (len(firsts[j]) - intersectionlen) * 2

        return ret
