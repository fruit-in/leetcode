class Solution:
    def maximumScore(self, scores: List[int], edges: List[List[int]]) -> int:
        n = len(scores)
        neighbors = [[] for _ in range(n)]
        top3 = [[] for _ in range(n)]
        ret = -1

        for a, b in edges:
            neighbors[a].append(b)
            neighbors[b].append(a)

        for a in range(n):
            top3[a] = sorted(neighbors[a], key=lambda b: scores[b])[-3:]

        for a, b in edges:
            top3a = [i for i in top3[a] if i != b]
            top3b = [i for i in top3[b] if i != a]

            if top3a == [] or top3b == []:
                continue
            elif top3a[-1] != top3b[-1]:
                ret = max(ret, scores[a] + scores[b] +
                          scores[top3a[-1]] + scores[top3b[-1]])
            else:
                if len(top3a) > 1:
                    ret = max(ret, scores[a] + scores[b] +
                              scores[top3a[-2]] + scores[top3b[-1]])
                if len(top3b) > 1:
                    ret = max(ret, scores[a] + scores[b] +
                              scores[top3a[-1]] + scores[top3b[-2]])

        return ret
