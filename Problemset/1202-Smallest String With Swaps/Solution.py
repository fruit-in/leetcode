class Solution:
    def smallestStringWithSwaps(self, s: str, pairs: List[List[int]]) -> str:
        parent = list(range(len(s)))
        groups = {}
        chars = []

        for a, b in pairs:
            while parent[a] != parent[parent[a]]:
                parent[a] = parent[parent[a]]
            while parent[b] != parent[parent[b]]:
                parent[b] = parent[parent[b]]

            if parent[a] < parent[b]:
                parent[parent[b]] = parent[a]
            else:
                parent[parent[a]] = parent[b]

        for i in range(len(s)):
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]

            if parent[i] not in groups:
                groups[parent[i]] = []
            groups[parent[i]].append(i)

        for group in groups:
            groups[group].sort(key=lambda i: -ord(s[i]))

        for i in range(len(s)):
            chars.append(s[groups[parent[i]].pop()])

        return ''.join(chars)
