class Solution:
    def numTilePossibilities(self, tiles: str) -> int:
        return len(self.dfs(tiles))

    def dfs(self, tiles: str) -> Set[str]:
        ret = set()

        for i in range(len(tiles)):
            ret.add(tiles[i])
            for s in self.dfs(tiles[:i] + tiles[i + 1:]):
                ret.add(s)
                ret.add(tiles[i] + s)

        return ret
