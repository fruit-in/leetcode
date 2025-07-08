class Solution:
    def longestRepeating(self, s: str, queryCharacters: str, queryIndices: List[int]) -> List[int]:
        size = 1 << ceil(log2(len(s)))
        treelength = [0] * (2 * size)
        treeleft = [('', 0)] * (2 * size)
        treeright = [('', 0)] * (2 * size)
        treerepeat = [0] * (2 * size)
        lengths = []

        for i, c in enumerate(s):
            treelength[size + i] = 1
            treeleft[size + i] = (c, 1)
            treeright[size + i] = (c, 1)
            treerepeat[size + i] = 1
        for i in range(size - 1, 0, -1):
            treelength[i] = treelength[2 * i] + treelength[2 * i + 1]
            treeleft[i] = treeleft[2 * i]
            treeright[i] = treeright[2 * i + 1]
            treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
            if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                treeleft[i] = (treeleft[i][0], treeleft[i]
                               [1] + treeleft[2 * i + 1][1])
            if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                treeright[i] = (treeright[i][0], treeright[i]
                                [1] + treeright[2 * i][1])
            if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                treerepeat[i] = max(
                    treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

        for c, i in zip(queryCharacters, queryIndices):
            i += size
            treeleft[i] = (c, 1)
            treeright[i] = (c, 1)
            while i > 1:
                i >>= 1
                treeleft[i] = treeleft[2 * i]
                treeright[i] = treeright[2 * i + 1]
                treerepeat[i] = max(treerepeat[2 * i], treerepeat[2 * i + 1])
                if treeleft[2 * i][1] == treelength[2 * i] and treeleft[2 * i][0] == treeleft[2 * i + 1][0]:
                    treeleft[i] = (treeleft[i][0], treeleft[i]
                                   [1] + treeleft[2 * i + 1][1])
                if treeright[2 * i + 1][1] == treelength[2 * i + 1] and treeright[2 * i + 1][0] == treeright[2 * i][0]:
                    treeright[i] = (treeright[i][0], treeright[i]
                                    [1] + treeright[2 * i][1])
                if treeright[2 * i][0] == treeleft[2 * i + 1][0]:
                    treerepeat[i] = max(
                        treerepeat[i], treeright[2 * i][1] + treeleft[2 * i + 1][1])

            lengths.append(treerepeat[1])

        return lengths
