# @param {Integer} n
# @return {Integer}
def total_money(n)
  x = n / 7
  y = n % 7

  28 * x + x * (x - 1) * 7 / 2 + (x + 1) * y + y * (y - 1) / 2
end
