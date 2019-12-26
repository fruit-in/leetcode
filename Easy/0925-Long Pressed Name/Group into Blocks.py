class Solution:
    def isLongPressedName(self, name: str, typed: str) -> bool:
        n_cnt = []
        cnt = 1
        for i in range(len(name)):
            if i < len(name) - 1 and name[i] == name[i + 1]:
                cnt += 1
            else:
                n_cnt.append((name[i], cnt))
                cnt = 1

        t_cnt = []
        cnt = 1
        for i in range(len(typed)):
            if i < len(typed) - 1 and typed[i] == typed[i + 1]:
                cnt += 1
            else:
                t_cnt.append((typed[i], cnt))
                cnt = 1

        return all(n[0] == t[0] and n[1] <= t[1] for n, t in zip(n_cnt, t_cnt)) and len(n_cnt) == len(t_cnt)
