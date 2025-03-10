class Solution:
    def matrixRankTransform(self, matrix: List[List[int]]) -> List[List[int]]:
        m, n = len(matrix), len(matrix[0])
        ranksr = [0] * m
        ranksc = [0] * n
        cells = sorted((matrix[r][c], r, c)
                       for r in range(m) for c in range(n))
        parent = {}
        answermap = {}

        for i in range(len(cells)):
            r, c = cells[i][1], cells[i][2]
            parent[(r, c)] = (r, c)
            answermap[(r, c)] = max(ranksr[r], ranksc[c]) + 1

            if i + 1 < len(cells) and cells[i][0] == cells[i + 1][0]:
                continue

            rowparent = {}
            colparent = {}

            for (r, c) in parent:
                if r not in rowparent:
                    rowparent[r] = (r, c)
                while rowparent[r] != parent[rowparent[r]]:
                    rowparent[r] = parent[rowparent[r]]
                if answermap[(r, c)] <= answermap[rowparent[r]]:
                    parent[(r, c)] = rowparent[r]
                else:
                    parent[rowparent[r]] = (r, c)
                if c not in colparent:
                    colparent[c] = (r, c)
                while colparent[c] != parent[colparent[c]]:
                    colparent[c] = parent[colparent[c]]
                if answermap[parent[(r, c)]] <= answermap[colparent[c]]:
                    parent[parent[(r, c)]] = colparent[c]
                else:
                    parent[colparent[c]] = parent[(r, c)]

            for (r, c) in parent:
                while parent[(r, c)] != parent[parent[(r, c)]]:
                    parent[(r, c)] = parent[parent[(r, c)]]
                answermap[(r, c)] = answermap[parent[(r, c)]]
                ranksr[r] = answermap[(r, c)]
                ranksc[c] = answermap[(r, c)]

            parent = {}

        return [[answermap[(r, c)] for c in range(n)] for r in range(m)]
