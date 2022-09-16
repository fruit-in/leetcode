class Solution:
    def bestHand(self, ranks: List[int], suits: List[str]) -> str:
        ranks_counter = Counter(ranks)
        suits_counter = Counter(suits)

        if len(suits_counter) == 1:
            return "Flush"
        elif max(ranks_counter.values()) > 2:
            return "Three of a Kind"
        elif max(ranks_counter.values()) > 1:
            return "Pair"
        else:
            return "High Card"
