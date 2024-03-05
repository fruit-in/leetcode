class Solution:
    def minSwapsCouples(self, row: List[int]) -> int:
        row = [person // 2 for person in row]
        parent = list(range(len(row) // 2))
        groups = {}

        for i in range(0, len(row), 2):
            if row[i] == row[i + 1]:
                continue

            while parent[parent[row[i]]] != parent[row[i]]:
                parent[row[i]] = parent[parent[row[i]]]
            while parent[parent[row[i + 1]]] != parent[row[i + 1]]:
                parent[row[i + 1]] = parent[parent[row[i + 1]]]

            if parent[row[i]] < parent[row[i + 1]]:
                parent[parent[row[i + 1]]] = parent[row[i]]
            else:
                parent[parent[row[i]]] = parent[row[i + 1]]

        for person in parent:
            while parent[parent[person]] != parent[person]:
                parent[person] = parent[parent[person]]

            groups[parent[person]] = groups.get(parent[person], 0) + 1

        return sum(x - 1 for x in groups.values())
