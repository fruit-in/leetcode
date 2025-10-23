class Solution:
    def sortItems(self, n: int, m: int, group: List[int], beforeitems: List[List[int]]) -> List[int]:
        groupitems = [[] for _ in range(m + 1)]
        afteritems = [[] for _ in range(n)]
        itemindegree = [0] * n
        groupindegree = [0] * m
        ret = []

        for i in range(len(group)):
            groupitems[group[i]].append(i)

            for j in beforeitems[i]:
                afteritems[j].append(i)
                itemindegree[i] += 1
                if group[i] != -1 and group[j] != group[i]:
                    groupindegree[group[i]] += 1

        nogroupstack = [i for i in groupitems[-1] if itemindegree[i] == 0]
        groupstack = [i for i in range(m) if groupindegree[i] == 0]

        while nogroupstack or groupstack:
            while nogroupstack:
                i = nogroupstack.pop()
                ret.append(i)

                for j in afteritems[i]:
                    itemindegree[j] -= 1
                    if itemindegree[j] == 0 and group[j] == -1:
                        nogroupstack.append(j)
                    if group[j] != -1:
                        groupindegree[group[j]] -= 1
                        if groupindegree[group[j]] == 0:
                            groupstack.append(group[j])

            while groupstack:
                g = groupstack.pop()
                itemstack = [i for i in groupitems[g] if itemindegree[i] == 0]
                temp = []

                while itemstack:
                    i = itemstack.pop()
                    temp.append(i)

                    for j in afteritems[i]:
                        itemindegree[j] -= 1
                        if itemindegree[j] == 0:
                            if group[j] == -1:
                                nogroupstack.append(j)
                            elif group[j] == g:
                                itemstack.append(j)
                        if group[j] != -1 and group[j] != g:
                            groupindegree[group[j]] -= 1
                            if groupindegree[group[j]] == 0:
                                groupstack.append(group[j])

                ret.extend(temp)

        return ret if len(ret) == n else []
