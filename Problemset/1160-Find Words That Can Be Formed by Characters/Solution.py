class Solution:
    def countCharacters(self, words: List[str], chars: str) -> int:
        ret = 0

        cnt = [0] * 26
        for ch in chars:
            cnt[ord(ch) - 97] += 1

        for word in words:
            cnt_copy = cnt[:]
            flag = True

            for ch in word:
                if not cnt_copy[ord(ch) - 97]:
                    flag = False
                    break
                cnt_copy[ord(ch) - 97] -= 1

            if flag:
                ret += len(word)

        return ret
