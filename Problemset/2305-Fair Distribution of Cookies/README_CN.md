# 2305. 公平分发饼干
给你一个整数数组 `cookies` ，其中 `cookies[i]` 表示在第 `i` 个零食包中的饼干数量。另给你一个整数 `k` 表示等待分发零食包的孩子数量，**所有** 零食包都需要分发。在同一个零食包中的所有饼干都必须分发给同一个孩子，不能分开。

分发的 **不公平程度** 定义为单个孩子在分发过程中能够获得饼干的最大总数。

返回所有分发的最小不公平程度。

#### 示例 1:
<pre>
<strong>输入:</strong> cookies = [8,15,10,20,8], k = 2
<strong>输出:</strong> 31
<strong>解释:</strong> 一种最优方案是 [8,15,8] 和 [10,20] 。
- 第 1 个孩子分到 [8,15,8] ，总计 8 + 15 + 8 = 31 块饼干。
- 第 2 个孩子分到 [10,20] ，总计 10 + 20 = 30 块饼干。
分发的不公平程度为 max(31,30) = 31 。
可以证明不存在不公平程度小于 31 的分发方案。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> cookies = [6,1,3,2,2,4,1,2], k = 3
<strong>输出:</strong> 7
<strong>解释:</strong> 一种最优方案是 [6,1]、[3,2,2] 和 [4,1,2] 。
- 第 1 个孩子分到 [6,1] ，总计 6 + 1 = 7 块饼干。
- 第 2 个孩子分到 [3,2,2] ，总计 3 + 2 + 2 = 7 块饼干。
- 第 3 个孩子分到 [4,1,2] ，总计 4 + 1 + 2 = 7 块饼干。
分发的不公平程度为 max(7,7,7) = 7 。
可以证明不存在不公平程度小于 7 的分发方案。
</pre>

#### 提示:
* `2 <= cookies.length <= 8`
* <code>1 <= cookies[i] <= 10<sup>5</sup></code>
* `2 <= k <= cookies.length`

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def distributeCookies(self, cookies: List[int], k: int) -> int:
        def dfs(i: int) -> None:
            nonlocal ret
            if i == len(cookies):
                ret = min(ret, max(children))
                return

            for j in range(k):
                if children[j] + cookies[i] < ret and (j == 0 or children[j] != children[j - 1]):
                    children[j] += cookies[i]
                    dfs(i + 1)
                    children[j] -= cookies[i]

        children = [0] * k
        children[0] = cookies[0]
        ret = max(max(cookies[:k - 1]), sum(cookies[k - 1:]))

        dfs(1)

        return ret
```
