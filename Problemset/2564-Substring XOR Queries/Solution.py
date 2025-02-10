class Solution:
    def substringXorQueries(self, s: str, queries: List[List[int]]) -> List[List[int]]:
        vals = {}

        for i in range(len(s)):
            if s[i] == '0':
                if 0 not in vals:
                    vals[0] = [i, i]
                continue

            val = 0

            for j in range(i, min(len(s), i + 32)):
                val = (val << 1) + int(s[j])
                if val not in vals:
                    vals[val] = [i, j]

        return [vals.get(first ^ second, [-1, -1]) for first, second in queries]
