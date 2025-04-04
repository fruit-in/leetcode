class Solution:
    MOD = 1000000007
    factorial = [1]

    def waysToBuildRooms(self, prevRoom: List[int]) -> int:
        n = len(prevRoom)
        indegree = [0] * n
        subtreenodes = [0] * n
        orders = [1] * n

        while len(self.factorial) < n + 1:
            self.factorial.append(
                self.factorial[-1] * len(self.factorial) % self.MOD)

        for i in range(1, n):
            indegree[prevRoom[i]] += 1

        nodes = [i for i in range(n) if indegree[i] == 0]

        while nodes[-1] != 0:
            i = nodes.pop()
            j = prevRoom[i]
            indegree[j] -= 1
            if indegree[j] == 0:
                nodes.append(j)
            subtreenodes[i] += 1
            subtreenodes[j] += subtreenodes[i]
            orders[j] = orders[j] * orders[i] % self.MOD
            orders[j] = orders[j] * self.factorial[subtreenodes[j]] % self.MOD
            orders[j] = orders[j] * \
                pow(self.factorial[subtreenodes[i]],
                    self.MOD - 2, self.MOD) % self.MOD
            orders[j] = orders[j] * pow(self.factorial[subtreenodes[j] -
                                        subtreenodes[i]], self.MOD - 2, self.MOD) % self.MOD

        return orders[0]
