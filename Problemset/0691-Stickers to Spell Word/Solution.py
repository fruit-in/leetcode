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
