class Solution:
    def findAllPeople(self, n: int, meetings: List[List[int]], firstPerson: int) -> List[int]:
        meetings.sort(key=lambda m: m[2])
        knows = {0, firstPerson}
        currknows = set()
        edges = {}

        for i in range(len(meetings)):
            x, y, time = meetings[i]

            if x in knows or y in knows:
                knows.add(x)
                knows.add(y)
                currknows.add(x)
                currknows.add(y)
            else:
                if x not in edges:
                    edges[x] = []
                if y not in edges:
                    edges[y] = []
                edges[x].append(y)
                edges[y].append(x)

            if i + 1 == len(meetings) or time != meetings[i + 1][2]:
                currknows = list(currknows)
                while currknows != []:
                    x = currknows.pop()
                    for y in edges.get(x, []):
                        if y not in knows:
                            knows.add(y)
                            currknows.append(y)
                currknows = set()
                edges = {}

        return list(knows)
