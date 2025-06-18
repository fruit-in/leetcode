# 691. 贴纸拼词
我们有 `n` 种不同的贴纸。每个贴纸上都有一个小写的英文单词。

您想要拼写出给定的字符串 `target` ，方法是从收集的贴纸中切割单个字母并重新排列它们。如果你愿意，你可以多次使用每个贴纸，每个贴纸的数量是无限的。

返回你需要拼出 `target` 的最小贴纸数量。如果任务不可能，则返回 `-1` 。

**注意：**在所有的测试用例中，所有的单词都是从 `1000` 个最常见的美国英语单词中随机选择的，并且 `target` 被选择为两个随机单词的连接。

#### 示例 1:
<pre>
<strong>输入:</strong> stickers = ["with","example","science"], target = "thehat"
<strong>输出:</strong> 3
<strong>解释:</strong>
我们可以使用 2 个 "with" 贴纸，和 1 个 "example" 贴纸。
把贴纸上的字母剪下来并重新排列后，就可以形成目标 “thehat“ 了。
此外，这是形成目标字符串所需的最小贴纸数量。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> stickers = ["notice","possible"], target = "basicbasic"
<strong>输出:</strong> -1
<strong>解释:</strong> 我们不能通过剪切给定贴纸的字母来形成目标“basicbasic”。
</pre>

#### 提示:
* `n == stickers.length`
* `1 <= n <= 50`
* `1 <= stickers[i].length <= 10`
* `1 <= target.length <= 15`
* `stickers[i]` 和 `target` 由小写英文单词组成

## 题解 (Python)

### 1. 题解
```Python
class Solution:
    def minStickers(self, stickers: List[str], target: str) -> int:
        @cache
        def dfs(mask: int) -> int:
            if mask == 0:
                return 0

            ret = inf

            for sticker in stickers:
                counter = Counter(sticker)
                newmask = mask

                for i in range(len(target)):
                    if (mask >> i) & 1 == 1 and counter.get(target[i], 0) > 0:
                        counter[target[i]] -= 1
                        newmask ^= 1 << i

                if newmask != mask:
                    ret = min(ret, dfs(newmask) + 1)

            return ret

        if any(all(ch not in sticker for sticker in stickers) for ch in target):
            return -1

        return dfs((1 << len(target)) - 1)
```
