# 691. Stickers to Spell Word
We are given `n` different types of `stickers`. Each sticker has a lowercase English word on it.

You would like to spell out the given string `target` by cutting individual letters from your collection of stickers and rearranging them. You can use each sticker more than once if you want, and you have infinite quantities of each sticker.

Return *the minimum number of stickers that you need to spell out* `target`. If the task is impossible, return `-1`.

**Note:** In all test cases, all words were chosen randomly from the `1000` most common US English words, and `target` was chosen as a concatenation of two random words.

#### Example 1:
<pre>
<strong>Input:</strong> stickers = ["with","example","science"], target = "thehat"
<strong>Output:</strong> 3
<strong>Explanation:</strong>
We can use 2 "with" stickers, and 1 "example" sticker.
After cutting and rearrange the letters of those stickers, we can form the target "thehat".
Also, this is the minimum number of stickers necessary to form the target string.
</pre>

#### Example 2:
<pre>
<strong>Input:</strong> stickers = ["notice","possible"], target = "basicbasic"
<strong>Output:</strong> -1
<strong>Explanation:</strong>
We cannot form the target "basicbasic" from cutting letters from the given stickers.
</pre>

#### Constraints:
* `n == stickers.length`
* `1 <= n <= 50`
* `1 <= stickers[i].length <= 10`
* `1 <= target.length <= 15`
* `stickers[i]` and `target` consist of lowercase English letters.

## Solutions (Python)

### 1. Solution
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
