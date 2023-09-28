# 1079. 活字印刷
你有一套活字字模 `tiles`，其中每个字模上都刻有一个字母 `tiles[i]`。返回你可以印出的非空字母序列的数目。

**注意：**本题中，每个活字字模只能使用一次。

#### 示例 1:
<pre>
<strong>输入:</strong> tiles = "AAB"
<strong>输出:</strong> 8
<strong>解释:</strong> 可能的序列为 "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> tiles = "AAABBC"
<strong>输出:</strong> 188
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> tiles = "V"
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= tiles.length <= 7`
* `tiles` 由大写英文字母组成

## 题解 (Python)

### 1. 题解
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
