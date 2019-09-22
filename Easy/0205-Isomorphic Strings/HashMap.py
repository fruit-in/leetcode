class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        smap = {}
        tset = set()

        for cs, ct in zip(s, t):
            if (cs in smap) != (ct in tset):
                return False
            elif cs not in smap:
                smap[cs] = ct
                tset.add(ct)
            elif smap[cs] != ct:
                return False

        return True
