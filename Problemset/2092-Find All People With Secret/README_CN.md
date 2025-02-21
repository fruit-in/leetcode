# 2092. 找出知晓秘密的所有专家
给你一个整数 `n` ，表示有 `n` 个专家从 `0` 到 `n - 1` 编号。另外给你一个下标从 0 开始的二维整数数组 `meetings` ，其中 <code>meetings[i] = [x<sub>i</sub>, y<sub>i</sub>, time<sub>i</sub>]</code> 表示专家 <code>x<sub>i</sub></code> 和专家 <code>y<sub>i</sub></code> 在时间 <code>time<sub>i</sub></code> 要开一场会。一个专家可以同时参加 **多场会议** 。最后，给你一个整数 `firstPerson` 。

专家 `0` 有一个 **秘密** ，最初，他在时间 `0` 将这个秘密分享给了专家 `firstPerson` 。接着，这个秘密会在每次有知晓这个秘密的专家参加会议时进行传播。更正式的表达是，每次会议，如果专家 <code>x<sub>i</sub></code> 在时间 <code>time<sub>i</sub></code> 时知晓这个秘密，那么他将会与专家 <code>y<sub>i</sub></code> 分享这个秘密，反之亦然。

秘密共享是 **瞬时发生** 的。也就是说，在同一时间，一个专家不光可以接收到秘密，还能在其他会议上与其他专家分享。

在所有会议都结束之后，返回所有知晓这个秘密的专家列表。你可以按 **任何顺序** 返回答案。

#### 示例 1:
<pre>
<strong>输入:</strong> n = 6, meetings = [[1,2,5],[2,3,8],[1,5,10]], firstPerson = 1
<strong>输出:</strong> [0,1,2,3,5]
<strong>解释:</strong>
时间 0 ，专家 0 将秘密与专家 1 共享。
时间 5 ，专家 1 将秘密与专家 2 共享。
时间 8 ，专家 2 将秘密与专家 3 共享。
时间 10 ，专家 1 将秘密与专家 5 共享。
因此，在所有会议结束后，专家 0、1、2、3 和 5 都将知晓这个秘密。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> n = 4, meetings = [[3,1,3],[1,2,2],[0,3,3]], firstPerson = 3
<strong>输出:</strong> [0,1,3]
<strong>解释:</strong>
时间 0 ，专家 0 将秘密与专家 3 共享。
时间 2 ，专家 1 与专家 2 都不知晓这个秘密。
时间 3 ，专家 3 将秘密与专家 0 和专家 1 共享。
因此，在所有会议结束后，专家 0、1 和 3 都将知晓这个秘密。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> n = 5, meetings = [[3,4,2],[1,2,1],[2,3,1]], firstPerson = 1
<strong>输出:</strong> [0,1,2,3,4]
<strong>解释:</strong>
时间 0 ，专家 0 将秘密与专家 1 共享。
时间 1 ，专家 1 将秘密与专家 2 共享，专家 2 将秘密与专家 3 共享。
注意，专家 2 可以在收到秘密的同一时间分享此秘密。
时间 2 ，专家 3 将秘密与专家 4 共享。
因此，在所有会议结束后，专家 0、1、2、3 和 4 都将知晓这个秘密。
</pre>

#### 提示:
* <code>2 <= n <= 10<sup>5</sup></code>
* <code>1 <= meetings.length <= 10<sup>5</sup></code>
* `meetings[i].length == 3`
* <code>0 <= x<sub>i</sub>, y<sub>i</sub> <= n - 1</code>
* <code>x<sub>i</sub> != y<sub>i</sub></code>
* <code>1 <= time<sub>i</sub> <= 10<sup>5</sup></code>
* `1 <= firstPerson <= n - 1`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def findAllPeople(self, n: int, meetings: List[List[int]], firstPerson: int) -> List[int]:
        meetings.sort(key=lambda m: m[2])
        knows = {0, firstPerson}
        currknows = set()
        edges = {}

        for i in range(len(meetings)):
            x, y, time = meetings[i]

            if x in knows or y in knows:
                knows.add(x)
                knows.add(y)
                currknows.add(x)
                currknows.add(y)
            else:
                if x not in edges:
                    edges[x] = []
                if y not in edges:
                    edges[y] = []
                edges[x].append(y)
                edges[y].append(x)

            if i + 1 == len(meetings) or time != meetings[i + 1][2]:
                currknows = list(currknows)
                while currknows != []:
                    x = currknows.pop()
                    for y in edges.get(x, []):
                        if y not in knows:
                            knows.add(y)
                            currknows.append(y)
                currknows = set()
                edges = {}

        return list(knows)
```
