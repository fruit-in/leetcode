class Solution:
    def addBinary(self, a: str, b: str) -> str:
        i = -1
        c = '0'
        ret = ""
        while i >= -len(a) or i >= -len(b):
            if (i < -len(a) or a[i] == '0') and (i < -len(b) or b[i] == '0'):
                ret = c + ret
                c = '0'
            elif i >= -len(a) and a[i] == '1' and i >= -len(b) and b[i] == '1':
                ret = c + ret
                c = '1'
            elif c == '0':
                ret = '1' + ret
            else:
                ret = '0' + ret
            i -= 1
        if c == '1':
            ret = '1' + ret
        return ret
