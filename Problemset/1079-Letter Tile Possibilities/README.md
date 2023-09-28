# 1079. Letter Tile Possibilities
You have `n`  `tiles`, where each tile has one letter `tiles[i]` printed on it.

Return *the number of possible non-empty sequences of letters* you can make using the letters printed on those `tiles`.

#### Example 1:
<pre>
<strong>Input:</strong> tiles = "AAB"
<strong>Output:</strong> 8
<strong>Explanation:</strong> The possible sequences are "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA".
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> tiles = "AAABBC"
<strong>Output:</strong> 188
</pre>

#### Example 3:
<pre>
<strong>Input:</strong> tiles = "V"
<strong>Output:</strong> 1
</pre>

#### Constraints:
* `1 <= tiles.length <= 7`
* `tiles` consists of uppercase English letters.

## Solutions (Python)

### 1. Solution
```Python
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
```
