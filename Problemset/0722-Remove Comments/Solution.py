class Solution:
    def removeComments(self, source: List[str]) -> List[str]:
        source = list('\n'.join(source))
        flagline = False
        flagblock = False
        i = 0

        while i < len(source):
            if flagline:
                if source[i] == '\n':
                    flagline = False
                else:
                    source[i] = '\0'
            elif flagblock:
                if source[i:i + 2] == ['*', '/']:
                    source[i] = '\0'
                    flagblock = False
                    i += 1
                source[i] = '\0'
            elif source[i:i + 2] == ['/', '/']:
                source[i] = '\0'
                source[i + 1] = '\0'
                flagline = True
                i += 1
            elif source[i:i + 2] == ['/', '*']:
                source[i] = '\0'
                source[i + 1] = '\0'
                flagblock = True
                i += 1

            i += 1

        source = [ch for ch in source if ch != '\0']
        source = ''.join(source).split('\n')

        return [line for line in source if line != ""]
