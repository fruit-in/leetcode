# @param {Integer} a
# @param {Integer} b
# @param {Integer} c
# @return {Integer}
def maximum_score(a, b, c)
  m = [a, b, c].max
  s = [a, b, c].sum

  s <= 2 * m ? s - m : s / 2
end
