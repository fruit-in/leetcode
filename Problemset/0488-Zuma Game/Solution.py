from functools import cache


class Solution:
    COLORS = "RYBGW"
    INDICES = dict(zip(COLORS, range(5)))

    @cache
    def removeGroups(self, board: str) -> str:
        stack = []

        for i in range(len(board)):
            if stack != [] and board[i] != stack[-1][0]:
                if stack[-1][1] > 2:
                    stack.pop()
            if stack != [] and board[i] == stack[-1][0]:
                stack[-1][1] += 1
            else:
                stack.append([board[i], 1])

        if stack[-1][1] > 2:
            stack.pop()

        return ''.join(ball * count for ball, count in stack)

    @cache
    def backtrack(self, board: str, hand: (int, int, int, int, int)) -> int:
        if board == "":
            return 0

        ret = -1

        for i in range(5):
            if hand[i] == 0:
                continue

            newhand = list(hand)
            newhand[i] -= 1
            newhand = tuple(newhand)

            for j in range(len(board) + 1):
                if j < len(board) and board[j] == self.COLORS[i]:
                    continue
                if j > 0 and j < len(board) and board[j] != board[j - 1] and board[j] != self.COLORS[i] and board[j - 1] != self.COLORS[i]:
                    continue

                newboard = self.removeGroups(
                    board[:j] + self.COLORS[i] + board[j:])
                minballs = self.backtrack(newboard, newhand)
                if minballs != -1 and (ret == -1 or minballs + 1 < ret):
                    ret = minballs + 1

        return ret

    def findMinStep(self, board: str, hand: str) -> int:
        count = [0] * 5

        for ball in hand:
            count[self.INDICES[ball]] += 1

        return self.backtrack(board, tuple(count))
