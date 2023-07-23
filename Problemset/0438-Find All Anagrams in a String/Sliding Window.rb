# @param {String} s
# @param {String} p
# @return {Integer[]}
def find_anagrams(s, p)
  return [] if s.size < p.size

  count_p = [0] * 26
  count_s = [0] * 26
  ret = []

  (0...p.size).each do |i|
    count_p[p[i].ord - 97] += 1
    count_s[s[i].ord - 97] += 1
  end

  (0..s.size - p.size).each do |i|
    ret.push(i) if count_p == count_s
    if i + p.size < s.size
      count_s[s[i].ord - 97] -= 1
      count_s[s[i + p.size].ord - 97] += 1
    end
  end

  ret
end
