# @param {Integer} low
# @param {Integer} high
# @return {Integer[]}
def sequential_digits(low, high)
  ret = []

  (1..8).each do |x|
    while x <= high && x % 10 != 0
      ret.push(x) if x >= low
      x = x * 10 + x % 10 + 1
    end
  end

  ret.sort
end
