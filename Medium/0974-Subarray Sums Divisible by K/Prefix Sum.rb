# @param {Integer[]} a
# @param {Integer} k
# @return {Integer}
def subarrays_div_by_k(a, k)
  counter = { 0 => 1 }
  counter.default = 0
  sum = 0
  ret = 0

  a.each do |x|
    sum = (sum + x) % k
    ret += counter[sum]
    counter[sum] += 1
  end

  ret
end
