# @param {Integer} n
# @param {Integer} k
# @return {String}
def get_smallest_string(n, k)
  k == 26 * n ? 'z' * n : 'a' * (n - (k - n) / 25 - 1) + (97 + (k - n) % 25).chr + 'z' * ((k - n) / 25)
end
