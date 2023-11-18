from sortedcontainers import SortedList


class Solution:
    def closestRoom(self, rooms: List[List[int]], queries: List[List[int]]) -> List[int]:
        queries = list(enumerate(queries))
        roomids = SortedList()
        answer = [-1] * len(queries)

        rooms.sort(key=lambda x: x[1])
        queries.sort(key=lambda x: x[1][1], reverse=True)

        for (j, [prefered, minsize]) in queries:
            while len(rooms) > 0 and rooms[-1][1] >= minsize:
                roomids.add(rooms.pop()[0])
            if len(roomids) == 0:
                continue

            i = roomids.bisect_left(prefered)
            if i == 0:
                answer[j] = roomids[0]
            elif i == len(roomids):
                answer[j] = roomids[-1]
            elif abs(roomids[i] - prefered) < abs(roomids[i - 1] - prefered):
                answer[j] = roomids[i]
            else:
                answer[j] = roomids[i - 1]

        return answer
