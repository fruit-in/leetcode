class Solution:
    def groupStrings(self, words: List[str]) -> List[int]:
        counter = {}
        parent = {}
        groups = {}
        largestsize = 0

        for s1 in sorted(words, key=len):
            mask1 = reduce(lambda x, y: x | (1 << (ord(y) - 97)), s1, 0)
            if mask1 in counter:
                counter[mask1] += 1
                continue
            counter[mask1] = 1
            parent[mask1] = mask1

            for i in range(26):
                if (mask1 >> i) & 1 == 0:
                    continue

                mask2 = mask1 ^ (1 << i)
                if mask2 in counter:
                    while parent[mask2] != parent[parent[mask2]]:
                        parent[mask2] = parent[parent[mask2]]
                    parent[parent[mask2]] = mask1

                for j in range(26):
                    if (mask2 >> j) & 1 == 1:
                        continue

                    mask3 = mask2 ^ (1 << j)
                    if mask3 in counter:
                        while parent[mask3] != parent[parent[mask3]]:
                            parent[mask3] = parent[parent[mask3]]
                        parent[parent[mask3]] = mask1

        for mask, count in counter.items():
            while parent[mask] != parent[parent[mask]]:
                parent[mask] = parent[parent[mask]]
            groups[parent[mask]] = groups.get(parent[mask], 0) + count
            largestsize = max(largestsize, groups[parent[mask]])

        return [len(groups), largestsize]
