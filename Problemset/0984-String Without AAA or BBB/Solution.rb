# @param {Integer} a
# @param {Integer} b
# @return {String}
def str_without3a3b(a, b)
  more_ch, less_ch = a > b ? %w[a b] : %w[b a]
  more_cnt = [a, b].max
  less_cnt = [a, b].min

  part0 = more_ch * 2 + less_ch
  part1 = more_ch + less_ch
  part2 = more_ch

  x = [more_cnt - less_cnt, less_cnt].min
  y = [2 * less_cnt - more_cnt, 0].max
  z = [more_cnt - 2 * less_cnt, 0].max

  part0 * x + part1 * y + part2 * z
end
