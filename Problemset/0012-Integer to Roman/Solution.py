class Solution:
    def intToRoman(self, num: int) -> str:
        roman = (num // 1000) * 'M'
        roman += (num % 1000 // 500) * 'D'
        roman += (num % 500 // 100) * 'C'
        roman += (num % 100 // 50) * 'L'
        roman += (num % 50 // 10) * 'X'
        roman += (num % 10 // 5) * 'V'
        roman += (num % 5) * 'I'

        roman = roman.replace("DCCCC", "CM")
        roman = roman.replace("CCCC", "CD")
        roman = roman.replace("LXXXX", "XC")
        roman = roman.replace("XXXX", "XL")
        roman = roman.replace("VIIII", "IX")
        roman = roman.replace("IIII", "IV")

        return roman
