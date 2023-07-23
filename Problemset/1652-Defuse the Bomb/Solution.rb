# @param {Integer[]} code
# @param {Integer} k
# @return {Integer[]}
def decrypt(code, k)
  n = code.length
  s = k > 0 ? 1 : n + k
  e = k > 0 ? k + 1 : n
  sum = code[s...e].sum
  ret = Array.new(n)

  (0...n).each do |i|
    ret[i] = sum
    sum += code[(e + i) % n] - code[(s + i) % n]
  end

  ret
end
