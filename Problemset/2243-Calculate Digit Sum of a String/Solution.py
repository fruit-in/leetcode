class Solution:
    def digitSum(self, s: str, k: int) -> str:
        while len(s) > k:
            groups = []
            for i in range(0, len(s), k):
                groups.append(sum(int(d) for d in s[i:i + k]))
            s = ''.join(str(x) for x in groups)

        return s
