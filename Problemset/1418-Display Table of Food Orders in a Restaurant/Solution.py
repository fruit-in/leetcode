class Solution:
    def displayTable(self, orders: List[List[str]]) -> List[List[str]]:
        tables = set()
        foods = set()
        count = {}
        ret = []

        for _, tn, fi in orders:
            tables.add(tn)
            foods.add(fi)
            count[(tn, fi)] = count.get((tn, fi), 0) + 1

        ret.append(["Table"] + sorted(foods))

        for tn in sorted(tables, key=int):
            ret.append([tn])
            for fi in ret[0][1:]:
                ret[-1].append(str(count.get((tn, fi), 0)))

        return ret
