class Solution:
    def compress(self, chars: List[str]) -> int:
        r, w_char, w_pos = 0, 0, 0

        for r in range(len(chars)):
            if r == len(chars) - 1 or chars[r] != chars[r + 1]:
                s = chars[w_char] + (str(r - w_char + 1) if r > w_char else "")

                for ch in s:
                    chars[w_pos] = ch
                    w_pos += 1

                w_char = r + 1

        return w_pos
