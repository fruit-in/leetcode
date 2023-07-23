class Solution:
    def reorderLogFiles(self, logs: List[str]) -> List[str]:
        return sorted(logs, key=lambda x: (0, x.split(' ', 1)[::-1])
                      if x.split()[1].isalpha() else (1,))
