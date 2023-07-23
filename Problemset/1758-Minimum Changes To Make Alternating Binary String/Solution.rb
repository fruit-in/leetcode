# @param {String} s
# @return {Integer}
def min_operations(s)
  b = 0
  count = 0

  s.each_byte do |c|
    count += 1 if c != b + 48
    b = 1 - b
  end

  [count, s.size - count].min
end
