# @param {Integer} n
# @return {String[]}
def simplified_fractions(n)
  ret = []

  (2..n).each do |i|
    (1...i).each do |j|
      ret.push(format('%d/%d', j, i)) if i.gcd(j) == 1
    end
  end

  ret
end
