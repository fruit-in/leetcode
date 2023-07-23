# @param {Integer} num
# @return {String}
def int_to_roman(num)
    roman = 'M' * (num / 1000)
    roman += 'D' * (num % 1000 / 500)
    roman += 'C' * (num % 500 / 100)
    roman += 'L' * (num % 100 / 50)
    roman += 'X' * (num % 50 / 10)
    roman += 'V' * (num % 10 / 5)
    roman += 'I' * (num % 5)

    roman.sub!("DCCCC", "CM")
    roman.sub!("CCCC", "CD")
    roman.sub!("LXXXX", "XC")
    roman.sub!("XXXX", "XL")
    roman.sub!("VIIII", "IX")
    roman.sub!("IIII", "IV")

    return roman
end
