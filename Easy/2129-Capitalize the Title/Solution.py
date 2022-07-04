class Solution:
    def capitalizeTitle(self, title: str) -> str:
        return ' '.join(w.lower() if len(w) < 3 else w.capitalize() for w in title.split())
