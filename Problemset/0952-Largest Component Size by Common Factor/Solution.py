class Solution:
    primefactors = [[] for _ in range(100001)]

    for i in range(2, 100001):
        if not primefactors[i]:
            for j in range(i, 100001, i):
                primefactors[j].append(i)

    def largestComponentSize(self, nums: List[int]) -> int:
        parent = list(range(max(nums) + 1))
        count = {}

        for i in nums:
            while parent[i] != parent[parent[i]]:
                parent[i] = parent[parent[i]]
            count[parent[i]] = count.get(parent[i], 0) + 1

            for j in self.primefactors[i]:
                while parent[j] != parent[parent[j]]:
                    parent[j] = parent[parent[j]]
                if parent[j] != parent[i]:
                    count[parent[i]] += count.get(parent[j], 0)
                    parent[parent[j]] = parent[i]

        return max(count.values())
