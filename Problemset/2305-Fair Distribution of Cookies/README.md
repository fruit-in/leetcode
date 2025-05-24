# 2305. Fair Distribution of Cookies
You are given an integer array `cookies`, where `cookies[i]` denotes the number of cookies in the <code>i<sup>th</sup></code> bag. You are also given an integer `k` that denotes the number of children to distribute **all** the bags of cookies to. All the cookies in the same bag must go to the same child and cannot be split up.

The **unfairness** of a distribution is defined as the **maximum total** cookies obtained by a single child in the distribution.

Return *the **minimum** unfairness of all distributions*.

#### Example 1:
<pre>
<strong>Input:</strong> cookies = [8,15,10,20,8], k = 2
<strong>Output:</strong> 31
<strong>Explanation:</strong> One optimal distribution is [8,15,8] and [10,20]
- The 1st child receives [8,15,8] which has a total of 8 + 15 + 8 = 31 cookies.
- The 2nd child receives [10,20] which has a total of 10 + 20 = 30 cookies.
The unfairness of the distribution is max(31,30) = 31.
It can be shown that there is no distribution with an unfairness less than 31.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> cookies = [6,1,3,2,2,4,1,2], k = 3
<strong>Output:</strong> 7
<strong>Explanation:</strong> One optimal distribution is [6,1], [3,2,2], and [4,1,2]
- The 1st child receives [6,1] which has a total of 6 + 1 = 7 cookies.
- The 2nd child receives [3,2,2] which has a total of 3 + 2 + 2 = 7 cookies.
- The 3rd child receives [4,1,2] which has a total of 4 + 1 + 2 = 7 cookies.
The unfairness of the distribution is max(7,7,7) = 7.
It can be shown that there is no distribution with an unfairness less than 7.
</pre>

#### Constraints:
* `2 <= cookies.length <= 8`
* <code>1 <= cookies[i] <= 10<sup>5</sup></code>
* `2 <= k <= cookies.length`

## Solutions (Python)

### 1. Solution
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
