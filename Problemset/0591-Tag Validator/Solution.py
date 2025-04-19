class Solution:
    def isValid(self, code: str) -> bool:
        cdata = False
        tagstack = []
        i = 0

        while i < len(code):
            if cdata:
                if code[i:i + 3] == "]]>":
                    cdata = False
                    i += 2
            elif tagstack == [] and (code[i] != '<' or code[i:i + 2] in "</<!"):
                return False
            elif code[i:i + 9] == "<![CDATA[":
                cdata = True
                i += 8
            elif code[i:i + 2] == "</":
                for j in range(i + 2, i + 13):
                    if j >= len(code) or j == i + 12 or (j == i + 2 and code[j] == '>'):
                        return False
                    elif code[j] == '>':
                        if tagstack.pop() != code[i + 2:j]:
                            return False
                        if tagstack == [] and j != len(code) - 1:
                            return False
                        i = j
                        break
                    elif not code[j].isupper():
                        return False
            elif code[i] == '<':
                for j in range(i + 1, i + 12):
                    if j >= len(code) or j == i + 11 or (j == i + 1 and code[j] == '>'):
                        return False
                    elif code[j] == '>':
                        tagstack.append(code[i + 1:j])
                        i = j
                        break
                    elif not code[j].isupper():
                        return False

            i += 1

        return tagstack == []
