class Solution:
    def decodeMessage(self, key: str, message: str) -> str:
        table = {ord(' '): ord(' ')}

        for c in key:
            c = ord(c)
            if c not in table:
                table[c] = ord('a') + len(table) - 1

        return message.translate(table)
