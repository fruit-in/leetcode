class Solution:
    def countPoints(self, points: List[List[int]], queries: List[List[int]]) -> List[int]:
        answer = [0] * len(queries)

        for i in range(len(points)):
            xi, yi = points[i]
            for j in range(len(queries)):
                xj, yj, rj = queries[j]
                if (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj) <= rj * rj:
                    answer[j] += 1

        return answer
