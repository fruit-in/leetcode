class Solution:
    def findItinerary(self, tickets: List[List[str]]) -> List[str]:
        targets = {}
        stack = []
        used = set()
        itinerary = [("JFK", None, None)]

        for fr0m, to in tickets:
            if fr0m not in targets:
                targets[fr0m] = []
            if to not in targets:
                targets[to] = []
            targets[fr0m].append(to)
        for fr0m in targets:
            targets[fr0m].sort(reverse=True)

        while True:
            fr0m = itinerary[-1][0]

            for i in range(len(targets[fr0m])):
                if (fr0m, i) not in used:
                    if stack == [] \
                            or fr0m != stack[-1][0] \
                            or targets[fr0m][i] != targets[fr0m][stack[-1][1]] \
                            or len(itinerary) != stack[-1][2]:
                        stack.append((fr0m, i, len(itinerary)))

            while stack[-1][2] < len(itinerary):
                _, fr0m, i = itinerary.pop()
                used.remove((fr0m, i))

            fr0m, i, _ = stack.pop()
            used.add((fr0m, i))
            itinerary.append((targets[fr0m][i], fr0m, i))

            if len(itinerary) == len(tickets) + 1:
                return [airport for airport, _, _ in itinerary]
