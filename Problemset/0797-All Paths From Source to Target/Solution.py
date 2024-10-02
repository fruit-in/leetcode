class Solution:
    def allPathsSourceTarget(self, graph: List[List[int]]) -> List[List[int]]:
        paths = [[0]]
        ret = []

        while paths != []:
            path = paths.pop()

            if path[-1] == len(graph) - 1:
                ret.append(path)
                continue

            for j in graph[path[-1]]:
                paths.append(path + [j])

        return ret
