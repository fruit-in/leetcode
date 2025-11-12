class Solution:
    def shortestSuperstring(self, words: List[str]) -> str:
        @cache
        def shortestLength(mask: int, prev: int) -> int:
            if mask == (1 << n) - 1:
                return len(words[prev])

            ret = inf

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    ret = min(ret, len(words[prev]) + length)

            return ret

        n = len(words)
        overlap = {(n, i): 0 for i in range(n)}
        mask = 0
        prev = n
        parts = []

        for i in range(n):
            for j in range(i + 1, n):
                overlap[(i, j)] = 0
                overlap[(j, i)] = 0
                for k in range(1, min(len(words[i]), len(words[j]))):
                    if words[i][-k:] == words[j][:k]:
                        overlap[(i, j)] = k
                    if words[j][-k:] == words[i][:k]:
                        overlap[(j, i)] = k

        for _ in range(n):
            minlength = inf
            minlengthi = n

            for i in range(n):
                if (mask >> i) & 1 == 0:
                    length = shortestLength(
                        mask | (1 << i), i) - overlap[(prev, i)]
                    if length < minlength:
                        minlength = length
                        minlengthi = i

            parts.append(words[minlengthi][overlap[(prev, minlengthi)]:])
            mask |= 1 << minlengthi
            prev = minlengthi

        return ''.join(parts)
