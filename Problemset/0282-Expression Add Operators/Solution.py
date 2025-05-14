class Solution:
    def addOperators(self, num: str, target: int) -> List[str]:
        def dfs(i: int) -> None:
            if i == len(chars):
                expression = ''.join(chars)
                if pattern.search(expression) is None and eval(expression) == target:
                    ret.append(expression)
                return

            chars[i] = '+'
            dfs(i + 2)
            chars[i] = '-'
            dfs(i + 2)
            chars[i] = '*'
            dfs(i + 2)
            chars[i] = ''
            dfs(i + 2)

        chars = [num[0]]
        pattern = re.compile(r'(?<!\d)0\d+')
        ret = []

        for c in num[1:]:
            chars.append('')
            chars.append(c)

        dfs(1)

        return ret
