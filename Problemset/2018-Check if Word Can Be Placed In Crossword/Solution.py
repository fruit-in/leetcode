class Solution:
    def placeWordInCrossword(self, board: List[List[str]], word: str) -> bool:
        def rotate() -> None:
            nonlocal board
            board = [list(row)[::-1] for row in zip(*board)]

        def placeWord() -> bool:
            for row in board:
                for s in ''.join(row).split('#'):
                    if len(s) == len(word) and all(x == y or x == ' ' for x, y in zip(s, word)):
                        return True

            return False

        return placeWord() or rotate() or placeWord() or rotate() or placeWord() or rotate() or placeWord()
