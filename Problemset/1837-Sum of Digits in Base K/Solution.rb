# @param {Integer} n
# @param {Integer} k
# @return {Integer}
def sum_base(n, k)
  ret = 0

  while n > 0
    ret += n % k
    n /= k
  end

  ret
end
