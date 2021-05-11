# @param {String} p
# @return {Integer}
def find_substring_in_wrapround_string(p)
  p = p.bytes
  count = 1
  max_len = [0] * 26

  (0...p.size).each do |i|
    count = i == 0 || (p[i] + 26 - p[i - 1]) % 26 != 1 ? 1 : count + 1
    (0...[26, count].min).each do |j|
      k = p[i + 1 - count + j] - 97
      max_len[k] = [max_len[k], count - j].max
    end
  end

  max_len.sum
end
