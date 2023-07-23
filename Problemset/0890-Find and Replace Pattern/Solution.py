class Solution:
    def findAndReplacePattern(self, words: List[str], pattern: str) -> List[str]:
        ret = []

        for word in words:
            map_wp = {}
            map_pw = {}

            for wl, pl in zip(word, pattern):
                if wl not in map_wp and pl not in map_pw:
                    map_wp[wl] = pl
                    map_pw[pl] = wl
                elif wl not in map_wp or pl not in map_pw:
                    break
                elif map_wp[wl] != pl or map_pw[pl] != wl:
                    break
            else:
                ret.append(word)

        return ret
