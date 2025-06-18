# 1147. 段式回文
你会得到一个字符串 `text` 。你应该把它分成 `k` 个子字符串 <code>(subtext<sub>1</sub>, subtext<sub>2</sub>，…， subtext<sub>k</sub>)</code> ，要求满足:
* <code>subtext<sub>i</sub></code> 是 **非空** 字符串
* 所有子字符串的连接等于 `text` ( 即<code>subtext<sub>1</sub> + subtext<sub>2</sub> + ... + subtext<sub>k</sub> == text</code> )
* 对于所有 `i` 的有效值( 即 `1 <= i <= k` ) ，<code>subtext<sub>i</sub> == subtext<sub>k - i + 1</sub></code> 均成立

返回`k`可能最大值。

#### 示例 1:
<pre>
<strong>输入:</strong> text = "ghiabcdefhelloadamhelloabcdefghi"
<strong>输出:</strong> 7
<strong>解释:</strong> 我们可以把字符串拆分成 "(ghi)(abcdef)(hello)(adam)(hello)(abcdef)(ghi)"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> text = "merchant"
<strong>输出:</strong> 1
<strong>解释:</strong> 我们可以把字符串拆分成 "(merchant)"。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> text = "antaprezatepzapreanta"
<strong>输出:</strong> 11
<strong>解释:</strong> 我们可以把字符串拆分成 "(a)(nt)(a)(pre)(za)(tep)(za)(pre)(a)(nt)(a)"。
</pre>

#### 提示:
* `1 <= text.length <= 1000`
* `text` 仅由小写英文字符组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def longestDecomposition(self, text: str) -> int:
        @cache
        def dfs(start: int) -> int:
            n = len(text) - start * 2

            if n < 2:
                return n

            j = 0
            lps = [0] * n
            ret = 1

            for i in range(1, n):
                while j > 0 and text[start + i] != text[start + j]:
                    j = lps[j - 1]

                if text[start + i] == text[start + j]:
                    j += 1
                    lps[i] = j

            j = lps[-1] - 1
            while j >= 0:
                if text[start + n - 1] == text[start + j]:
                    ret = max(ret, 2 + dfs(start + j + 1))
                j = lps[j] - 1

            return ret

        return dfs(0)
```
