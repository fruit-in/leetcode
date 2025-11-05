class Solution:
    def pyramidTransition(self, bottom: str, allowed: List[str]) -> bool:
        @cache
        def buildFromBottom(bottom: str) -> bool:
            if len(bottom) == 1:
                return True

            top = [''] * (len(bottom) - 1)

            def buildTop(i: int) -> bool:
                if i >= len(top):
                    return buildFromBottom(''.join(top))

                for pattern in allowed:
                    if bottom[i:i + 2] == pattern[:2]:
                        top[i] = pattern[2]
                        if buildTop(i + 1):
                            return True
                        top[i] = ''

                return False

            return buildTop(0)

        return buildFromBottom(bottom)
