class Solution:
    digit_to_letters = {'2': 'abc', '3': 'def',
                        '4': 'ghi', '5': 'jkl',
                        '6': 'mno', '7': 'pqrs',
                        '8': 'tuv', '9': 'wxyz'}

    def letterCombinations(self, digits: str) -> List[str]:
        if not digits:
            return []
        if len(digits) == 1:
            return list(self.digit_to_letters[digits])

        sub = self.letterCombinations(digits[1:])

        return [c + s for s in sub for c in self.digit_to_letters[digits[0]]]
